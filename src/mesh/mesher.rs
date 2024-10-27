use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::{
    boundary::Grid,
    mesh::geometry::{self, Quad, Triangle, Vector},
};
use ndarray::{Array2, Array3};

#[derive(Clone)]
pub enum WallKind {
    Terrain,
    Sky,
    Inlet,
    Interior,
}

#[derive(Clone)]
pub enum Poly {
    Triangle(Triangle),
    Quad(Quad),
}

#[derive(Clone)]
pub struct Wall {
    poly: Poly,
    kind: WallKind,
    cells_id: [Option<usize>; 2],
}

/*
         3             2
         +-------------+  ---- x
        /|            /|
       / |           / |
     7/  |         6/  |
     +-------------+   |
    /|   |         |   |
   / |   +---------|---+
  /  |  /0         |  /1
 y   | / |         | /
     |/  |         |/
     +-------------+
     4   |         5
         |
         z
*/
#[derive(Clone)]
pub struct Cell {
    id: usize,
    vertices: Vec<Vector>,
    walls: Vec<Wall>,
    center: Vector,
    neighbours: Vec<usize>,
}

pub struct Mesh {
    cells: Vec<Cell>,
}

impl Wall {
    pub fn new(points: &[&Vector], kind: WallKind, cells_id: [Option<usize>; 2]) -> Wall {
        match points {
            [v1, v2, v3] => Wall {
                poly: Poly::Triangle(Triangle::new(v1, v2, v3)),
                kind,
                cells_id,
            },
            [v1, v2, v3, v4] => Wall {
                poly: Poly::Quad(Quad::new(v1, v2, v3, v4)),
                kind,
                cells_id,
            },
            _ => unreachable!("Invalid number of points for a wall"),
        }
    }
}

impl Mesh {
    pub fn naive_mesh(terrain: &Grid, zs: Vec<f64>) -> Mesh {
        // Assume zs are sorted
        let nx = terrain.nx;
        let ny = terrain.ny;
        let nz = zs.len();
        let zs: Vec<f64> = zs.into_iter().rev().collect();

        let mut cells: Array3<Option<Cell>> = Array3::from_elem((nx, ny, nz), None);
        let mut z_count = Array2::from_elem((nx, ny), 0);

        // Create cells
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                let min_height = [
                    terrain.z(i, j),
                    terrain.z(i + 1, j),
                    terrain.z(i, j + 1),
                    terrain.z(i + 1, j + 1),
                ]
                .into_iter()
                .reduce(f64::min)
                .unwrap();

                z_count[(i, j)] = 2 + zs
                    .iter()
                    .take_while(|&&z| z >= min_height)
                    .enumerate()
                    .map(|(i, _z)| i)
                    .last()
                    .unwrap();

                for k in 0..z_count[(i, j)] - 1 {
                    let Vector { x: xa, y: ya, z: _ } = terrain.xyz(i, j);
                    let Vector { x: xb, y: yb, z: _ } = terrain.xyz(i + 1, j);
                    let Vector { x: xc, y: yc, z: _ } = terrain.xyz(i, j + 1);
                    let Vector { x: xd, y: yd, z: _ } = terrain.xyz(i + 1, j + 1);

                    let z1 = zs[k];
                    let z2 = zs[k + 1];

                    let v0 = Vector::new(xa, ya, z2);
                    let v1 = Vector::new(xb, yb, z2);
                    let v2 = Vector::new(xb, yb, z1);
                    let v3 = Vector::new(xa, ya, z1);
                    let v4 = Vector::new(xc, yc, z2);
                    let v5 = Vector::new(xd, yd, z2);
                    let v6 = Vector::new(xd, yd, z1);
                    let v7 = Vector::new(xc, yc, z1);

                    let id = (nx * ny) * k + ny * i + j;
                    let vertices = vec![v0, v1, v2, v3, v4, v5, v6, v7];
                    let center = geometry::average_points(&vertices);
                    cells[(i, j, k)] = Some(Cell {
                        id,
                        vertices,
                        walls: Vec::with_capacity(6),
                        center,
                        neighbours: Vec::with_capacity(6),
                    });
                }
            }
        }
        /*
                 3             2
                 +-------------+  ---- x
                /|            /|
               / |           / |
             7/  |         6/  |
             +-------------+   |
            /|   |         |   |
           / |   +---------|---+
          /  |  /0         |  /1
         y   | / |         | /
             |/  |         |/
             +-------------+
             4   |         5
                 |
                 z
        */
        // Create walls
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                for k in 0..z_count[(i, j)] {
                    if let Some(cell) = &mut cells[(i, j, k)] {
                        let mut vertices = cell.vertices.iter();
                        let v0 = vertices.next().unwrap();
                        let v1 = vertices.next().unwrap();
                        let v2 = vertices.next().unwrap();
                        let v3 = vertices.next().unwrap();
                        let v4 = vertices.next().unwrap();
                        let v5 = vertices.next().unwrap();
                        let v6 = vertices.next().unwrap();
                        let v7 = vertices.next().unwrap();

                        let (kind, neighs) = if k == 0 {
                            (WallKind::Sky, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * (k - 1) + ny * i + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_upper = Wall::new(&[&v3, &v7, &v6, &v2], kind, neighs);
                        cell.walls.push(wall_upper);

                        let (kind, neighs) = if j == 0 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * i + (j - 1);
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_south = Wall::new(&[&v3, &v2, &v1, &v0], kind, neighs);
                        cell.walls.push(wall_south);

                        let (kind, neighs) = if i == 0 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * (i - 1) + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_west = Wall::new(&[&v0, &v4, &v7, &v3], kind, neighs);
                        cell.walls.push(wall_west);

                        let (kind, neighs) = if k == z_count[(i, j)] - 1 {
                            (WallKind::Terrain, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * (k + 1) + ny * i + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_lower = Wall::new(&[&v0, &v1, &v5, &v4], kind, neighs);
                        cell.walls.push(wall_lower);

                        let (kind, neighs) = if j == ny - 1 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * i + (j + 1);
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_north = Wall::new(&[&v4, &v5, &v6, &v7], kind, neighs);
                        cell.walls.push(wall_north);

                        let (kind, neighs) = if i == nx - 1 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * (i + 1) + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_east = Wall::new(&[&v1, &v2, &v6, &v5], kind, neighs);
                        cell.walls.push(wall_east);
                    }
                }
            }
        }

        // Compute new index to remove None cells
        let mut new_idx: Vec<usize> = vec![0; nx * ny * nz];
        let mut count = 0;
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                for k in 0..z_count[(i, j)] {
                    if let Some(cell) = &cells[(i, j, k)] {
                        new_idx[cell.id] = count;
                        count += 1;
                    }
                }
            }
        }

        // Reindex cells and walls
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                for k in 0..z_count[(i, j)] {
                    if let Some(cell) = &mut cells[(i, j, k)] {
                        cell.id = new_idx[cell.id];
                        for wall in cell.walls.clone().iter_mut() {
                            for maybe_cid in wall.cells_id.iter_mut() {
                                if let Some(cell_id) = maybe_cid {
                                    *cell_id = new_idx[*cell_id];
                                }
                            }
                        }
                    }
                }
            }
        }

        let cells_mesh: Vec<Cell> = cells.into_iter().filter_map(|c| c).collect();
        Mesh { cells: cells_mesh }
    }

    pub fn save_to_vtk(&self, filename: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let file = File::create(filename)?;
        let mut file = BufWriter::new(file);

        writeln!(file, "# vtk DataFile Version 3.0")?;
        writeln!(file, "Mesh exported from Rust")?;
        writeln!(file, "ASCII")?;
        writeln!(file, "DATASET UNSTRUCTURED_GRID")?;

        // Write points
        let mut points = Vec::new();
        for cell in self.cells.iter() {
            points.extend(cell.vertices.clone());
        }
        writeln!(file, "POINTS {} float", points.len())?;
        for point in &points {
            writeln!(file, "{} {} {}", point.x, point.y, point.z)?;
        }

        // Write cells
        let total_cells = self.cells.len();
        let size = self
            .cells
            .iter()
            .map(|c| c.vertices.len() + 1)
            .sum::<usize>();
        writeln!(file, "CELLS {} {}", total_cells, size)?;
        let mut point_offset = 0;
        for cell in &self.cells {
            write!(file, "{}", cell.vertices.len())?;
            for i in 0..cell.vertices.len() {
                write!(file, " {}", point_offset + i)?;
            }
            writeln!(file)?;
            point_offset += cell.vertices.len();
        }

        // Write cell types
        writeln!(file, "CELL_TYPES {}", total_cells)?;
        for cell in &self.cells {
            let vtk_type = match cell.vertices.len() {
                4 => 10, // VTK_TETRA
                8 => 12, // VTK_HEXAHEDRON
                _ => 7,  // VTK_POLYGON (generic)
            };
            writeln!(file, "{}", vtk_type)?;
        }

        // Part 5: Dataset attributes (optional)
        writeln!(file, "CELL_DATA {}", total_cells)?;

        // Write cell IDs
        writeln!(file, "SCALARS cell_id int 1")?;
        writeln!(file, "LOOKUP_TABLE default")?;
        for cell in &self.cells {
            writeln!(file, "{}", cell.id)?;
        }

        Ok(())
    }
}
