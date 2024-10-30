use crate::math;
use crate::{
    boundary::Grid,
    mesh::geometry::{self, Quad, Triangle, Vector},
    sparse_system::sparse_matrix::SparseMatrix,
    sparse_system::sparse_system::SparseSystem,
};
use ndarray::{Array2, Array3};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const DENSITY_LAPSE_RATE: f64 = -0.0013;
const TEMPERATURE_LAPSE_RATE: f64 = -0.0065;
const UNIVERSAL_GAS_CONSTANT: f64 = 8.31432;
const GRAVITY: f64 = 9.80665;
const AIR_MOLAR_MASS: f64 = 0.0289644;
const PRESSURE_SEA_LEVEL: f64 = 101325.0;
const TEMPERATURE_SEA_LEVEL: f64 = 20.0 + 273.15;
const CALORIFIC_CAPACITY_V: f64 = 1214.0;

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
    pub poly: Poly,
    pub kind: WallKind,
    pub cells_id: [Option<usize>; 2],
    pub center: Vector,
    pub physics: Physics,
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
    pub id: usize,
    pub vertices: Vec<Vector>,
    pub walls: Vec<Wall>,
    pub center: Vector,
    pub neighbours: Vec<usize>,
    pub physics: Physics,
    pub ground_height: f64,
    pub volume: f64,
}

#[derive(Clone)]
pub struct Physics {
    pub velocity: Vector,
    pub pressure: f64,
    pub temperature: f64,
    pub density: f64,
    pub energy: f64,
}

pub struct Mesh {
    pub cells: Vec<Cell>,
}

pub struct InitialPhysics {
    pub z_ref: f64,
    pub speed_ref: f64,
    pub density_ref: f64,
    pub direction: f64,
    pub shear: f64,
    pub temperature: f64,
}

impl Physics {
    pub fn new() -> Physics {
        Physics {
            velocity: Vector::new(0.0, 0.0, 0.0),
            pressure: 0.0,
            temperature: 0.0,
            density: 0.0,
            energy: 0.0,
        }
    }

    pub fn from_inital_conditions(init_conds: &InitialPhysics, height: f64) -> Physics {
        let press_power =
            -GRAVITY * AIR_MOLAR_MASS / (UNIVERSAL_GAS_CONSTANT * TEMPERATURE_LAPSE_RATE);
        let delta_z = height - init_conds.z_ref;
        let density = init_conds.density_ref * delta_z * DENSITY_LAPSE_RATE;
        let temperature = init_conds.temperature;
        let pressure = PRESSURE_SEA_LEVEL
            * (1.0 + TEMPERATURE_LAPSE_RATE / TEMPERATURE_SEA_LEVEL * height).powf(press_power);

        let u_ref = init_conds.speed_ref * math::as_rads(init_conds.direction).cos();
        let v_ref = init_conds.speed_ref * math::as_rads(init_conds.direction).sin();
        let u = u_ref * (height / init_conds.z_ref).powf(init_conds.shear);
        let v = v_ref * (height / init_conds.z_ref).powf(init_conds.shear);

        let energy = 0.5 * (u * u + v * v) + CALORIFIC_CAPACITY_V * temperature;

        Physics {
            velocity: Vector::new(u, v, 0.0),
            pressure,
            temperature,
            density,
            energy,
        }
    }
}

impl Wall {
    pub fn new(points: &[&Vector], kind: WallKind, cells_id: [Option<usize>; 2]) -> Wall {
        let owned_points: Vec<Vector> = points.iter().map(|&&v| v.clone()).collect();
        let center = geometry::average_points(&owned_points);

        match points {
            [v1, v2, v3] => Wall {
                poly: Poly::Triangle(Triangle::new(v1, v2, v3)),
                kind,
                cells_id,
                center,
                physics: Physics::new(),
            },
            [v1, v2, v3, v4] => Wall {
                poly: Poly::Quad(Quad::new(v1, v2, v3, v4)),
                kind,
                cells_id,
                center,
                physics: Physics::new(),
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

                let avg_height = [
                    terrain.z(i, j),
                    terrain.z(i + 1, j),
                    terrain.z(i, j + 1),
                    terrain.z(i + 1, j + 1),
                ]
                .into_iter()
                .sum::<f64>()
                    / 4.0;

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
                    let volume = (v1.x - v0.x) * (v4.y - v0.y) * (v3.z - v0.z);

                    cells[(i, j, k)] = Some(Cell {
                        id,
                        vertices,
                        walls: Vec::with_capacity(6),
                        center,
                        neighbours: Vec::with_capacity(6),
                        physics: Physics::new(),
                        ground_height: avg_height,
                        volume,
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

        // Write velocity components
        writeln!(file, "VECTORS velocity float")?;
        for cell in &self.cells {
            writeln!(
                file,
                "{} {} {}",
                cell.physics.velocity.x, cell.physics.velocity.y, cell.physics.velocity.z
            )?;
        }

        // Write pressure
        writeln!(file, "SCALARS pressure float 1")?;
        writeln!(file, "LOOKUP_TABLE default")?;
        for cell in &self.cells {
            writeln!(file, "{}", cell.physics.pressure)?;
        }

        // Write temperature
        writeln!(file, "SCALARS temperature float 1")?;
        writeln!(file, "LOOKUP_TABLE default")?;
        for cell in &self.cells {
            writeln!(file, "{}", cell.physics.temperature)?;
        }

        // Write density
        writeln!(file, "SCALARS density float 1")?;
        writeln!(file, "LOOKUP_TABLE default")?;
        for cell in &self.cells {
            writeln!(file, "{}", cell.physics.density)?;
        }

        Ok(())
    }

    pub fn define_initial_and_boundary_conditions(&mut self, initial_physics: InitialPhysics) {
        self.cells.par_iter_mut().for_each(|cell| {
            cell.physics = Physics::from_inital_conditions(&initial_physics, cell.center.z);

            for wall in cell.walls.iter_mut() {
                let mut physics = Physics::from_inital_conditions(&initial_physics, wall.center.z);
                wall.physics = match wall.kind {
                    WallKind::Terrain => {
                        physics.velocity.x = 0.0;
                        physics.velocity.y = 0.0;
                        physics.velocity.z = 0.0;
                        physics
                    }
                    WallKind::Sky => {
                        physics.velocity.z = 0.0;
                        physics
                    }
                    _ => physics,
                }
            }
        })
    }

    pub fn make_system(&self) -> SparseSystem {
        todo!();
    }
}
