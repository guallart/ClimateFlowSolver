use std::fs::File;
use std::io::{BufWriter, Write};

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
         1             2
         +-------------+
        /|            /|
       / |           / |
     3/  |         4/  |
     +-------------+   |
     |   |         |   |
     |   +---------|---+
     |  /5         |  /6
     | /           | /
     |/            |/
     +-------------+
     7             8
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
        let mut depth = Array2::from_elem((nx, ny), 0);

        // Create cells
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                let max_height = [
                    terrain.z(i, j),
                    terrain.z(i + 1, j),
                    terrain.z(i, j + 1),
                    terrain.z(i + 1, j + 1),
                ]
                .into_iter()
                .reduce(f64::max)
                .unwrap();

                depth[(i, j)] = 1 + zs.partition_point(|z| *z >= max_height);

                for k in 0..depth[(i, j)] - 1 {
                    let Vector { x: x1, y: y1, z: _ } = terrain.xyz(i, j);
                    let Vector { x: x2, y: y2, z: _ } = terrain.xyz(i + 1, j);
                    let Vector { x: x3, y: y3, z: _ } = terrain.xyz(i, j + 1);
                    let Vector { x: x4, y: y4, z: _ } = terrain.xyz(i + 1, j + 1);

                    let z1 = zs[k];
                    let z2 = zs[k + 1];

                    let v1 = Vector::new(x1, y1, z1);
                    let v2 = Vector::new(x2, y2, z1);
                    let v3 = Vector::new(x3, y3, z1);
                    let v4 = Vector::new(x4, y4, z1);
                    let v5 = Vector::new(x1, y1, z2);
                    let v6 = Vector::new(x2, y2, z2);
                    let v7 = Vector::new(x3, y3, z2);
                    let v8 = Vector::new(x4, y4, z2);

                    let id = (nx * ny) * k + ny * i + j;
                    let vertices = vec![v1, v2, v3, v4, v5, v6, v7, v8];
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
                 1             2
                 +-------------+  ---- x
                /|            /|
               / |           / |
             3/  |         4/  |
             +-------------+   |
            /|   |         |   |
           / |   +---------|---+
          /  |  /5         |  /6
         y   | / |         | /
             |/  |         |/
             +-------------+
             7   |         8
                 |
                 z
        */
        // Create walls
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                for k in 0..depth[(i, j)] {
                    if let Some(cell) = &mut cells[(i, j, k)] {
                        let mut vertices = cell.vertices.iter();
                        let v1 = vertices.next().unwrap();
                        let v2 = vertices.next().unwrap();
                        let v3 = vertices.next().unwrap();
                        let v4 = vertices.next().unwrap();
                        let v5 = vertices.next().unwrap();
                        let v6 = vertices.next().unwrap();
                        let v7 = vertices.next().unwrap();
                        let v8 = vertices.next().unwrap();

                        let (kind, neighs) = if k == 0 {
                            (WallKind::Sky, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * (k - 1) + ny * i + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_upper = Wall::new(&[&v1, &v2, &v3, &v4], kind, neighs);
                        cell.walls.push(wall_upper);

                        let (kind, neighs) = if j == 0 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * i + (j - 1);
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_south = Wall::new(&[&v1, &v2, &v5, &v6], kind, neighs);
                        cell.walls.push(wall_south);

                        let (kind, neighs) = if i == 0 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * (i - 1) + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_west = Wall::new(&[&v1, &v3, &v5, &v7], kind, neighs);
                        cell.walls.push(wall_west);

                        let (kind, neighs) = if k == depth[(i, j)] - 1 {
                            (WallKind::Terrain, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * (k + 1) + ny * i + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_lower = Wall::new(&[&v5, &v6, &v7, &v8], kind, neighs);
                        cell.walls.push(wall_lower);

                        let (kind, neighs) = if j == ny - 1 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * i + (j + 1);
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_north = Wall::new(&[&v3, &v4, &v7, &v8], kind, neighs);
                        cell.walls.push(wall_north);

                        let (kind, neighs) = if i == nx - 1 {
                            (WallKind::Inlet, [Some(cell.id), None])
                        } else {
                            let neigh_id = (nx * ny) * k + ny * (i + 1) + j;
                            (WallKind::Interior, [Some(cell.id), Some(neigh_id)])
                        };
                        let wall_east = Wall::new(&[&v2, &v4, &v6, &v8], kind, neighs);
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
                for k in 0..depth[(i, j)] {
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
                for k in 0..depth[(i, j)] {
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

    pub fn save_to_vtk(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;

        writeln!(file, "# vtk DataFile Version 3.0")?;
        writeln!(file, "Mesh exported from ClimateFlowSolver")?;
        writeln!(file, "ASCII")?;
        writeln!(file, "DATASET UNSTRUCTURED_GRID")?;

        // write vertices
        let mut points = Vec::new();
        let mut point_index = 0;
        let mut point_map = std::collections::HashMap::new();

        for cell in &self.cells {
            for vertex in &cell.vertices {
                if !point_map.contains_key(vertex) {
                    point_map.insert(vertex, point_index);
                    points.push(vertex);
                    point_index += 1;
                }
            }
        }

        writeln!(file, "POINTS {} float", points.len())?;
        for point in points {
            writeln!(file, "{} {} {}", point.x, point.y, point.z)?;
        }

        // Write cells
        let total_size: usize = self.cells.iter().map(|c| c.vertices.len() + 1).sum();
        writeln!(file, "CELLS {} {}", self.cells.len(), total_size)?;

        for cell in &self.cells {
            write!(file, "{}", cell.vertices.len())?;
            for vertex in &cell.vertices {
                write!(file, " {}", point_map[vertex])?;
            }
            writeln!(file)?;
        }

        // Write cell types
        writeln!(file, "CELL_TYPES {}", self.cells.len())?;
        for _cell in &self.cells {
            writeln!(file, "12")?; //all hexahedral
        }

        // write cell data
        writeln!(file, "CELL_DATA {}", self.cells.len())?;

        // write cell ids
        writeln!(file, "SCALARS cell_id int 1")?;
        writeln!(file, "LOOKUP_TABLE default")?;
        for cell in &self.cells {
            writeln!(file, "{}", cell.id)?;
        }

        // write walls types
        writeln!(file, "FIELD FieldData 1")?;
        writeln!(file, "wall_types 1 {} int", self.cells.len())?;
        for cell in &self.cells {
            let wall_type = cell
                .walls
                .iter()
                .map(|w| match w.kind {
                    WallKind::Terrain => 0,
                    WallKind::Sky => 1,
                    WallKind::Inlet => 2,
                    WallKind::Interior => 3,
                })
                .next()
                .unwrap();
            writeln!(file, "{}", wall_type)?;
        }

        Ok(())
    }
}
