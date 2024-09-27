use ndarray::Array2;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Triangle {
    normal: Vector,
    vertices: [Vector; 3],
}

pub fn compute_normal(v1: &Vector, v2: &Vector, v3: &Vector) -> Vector {
    let u = Vector {
        x: v2.x - v1.x,
        y: v2.y - v1.y,
        z: v2.z - v1.z,
    };
    let v = Vector {
        x: v3.x - v1.x,
        y: v3.y - v1.y,
        z: v3.z - v1.z,
    };

    let normal = Vector {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };

    let mag = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    Vector {
        x: normal.x / mag,
        y: normal.y / mag,
        z: normal.z / mag,
    }
}

impl Triangle {
    fn new(v1: &Vector, v2: &Vector, v3: &Vector) -> Triangle {
        Triangle {
            vertices: [v1.clone(), v2.clone(), v3.clone()],
            normal: compute_normal(v1, v2, v3),
        }
    }
}

pub struct Grid {
    pub points: Array2<f64>,
    pub x_min: f64,
    pub y_min: f64,
    pub x_res: f64,
    pub y_res: f64,
    pub x_size: usize,
    pub y_size: usize,
}

impl Grid {
    fn get_xyz(&self, i: usize, j: usize) -> Vector {
        Vector {
            x: self.x_min + self.x_res * (i * self.x_size) as f64,
            y: self.y_min + self.y_res * (j * self.y_size) as f64,
            z: self.points[[i, j]],
        }
    }
}

pub fn triangulate_grid(terrain: &Grid) -> Vec<Triangle> {
    let mut triangles = Vec::new();
    let (rows, cols) = terrain.points.dim();

    for r in 0..rows - 1 {
        for c in 0..cols - 1 {
            let v1 = terrain.get_xyz(c, r);
            let v2 = terrain.get_xyz(c + 1, r);
            let v3 = terrain.get_xyz(c, r + 1);
            let v4 = terrain.get_xyz(c + 1, r + 1);

            triangles.push(Triangle::new(&v1, &v2, &v3));
            triangles.push(Triangle::new(&v2, &v3, &v4));
        }
    }

    triangles
}

pub fn make_walls(
    terrain: &Grid,
    max_heigh: f64,
) -> (
    Vec<Triangle>,
    Vec<Triangle>,
    Vec<Triangle>,
    Vec<Triangle>,
    Vec<Triangle>,
) {
    let mut north: Vec<Triangle> = Vec::new();
    let mut south: Vec<Triangle> = Vec::new();
    let mut east: Vec<Triangle> = Vec::new();
    let mut west: Vec<Triangle> = Vec::new();
    let mut sky: Vec<Triangle> = Vec::new();

    let sw_up = terrain.get_xyz(0, 0);
    let nw_up = terrain.get_xyz(0, terrain.y_size - 1);
    let ne_up = terrain.get_xyz(terrain.x_size - 1, 0);
    let se_up = terrain.get_xyz(terrain.x_size - 1, terrain.y_size - 1);

    todo!();

    (north, south, east, west, sky)
}

pub fn write(triangles: Vec<Triangle>, file_name: &str) -> Result<(), std::io::Error> {
    let mut stl_file = File::create(file_name)?;
    writeln!(stl_file, "solid Vec<Triangle>")?;
    for triangle in triangles {
        writeln!(
            stl_file,
            "  facet normal {:.6} {:.6} {:.6}",
            triangle.normal.x, triangle.normal.y, triangle.normal.z
        )?;
        writeln!(stl_file, "    outer loop")?;
        for vertex in &triangle.vertices {
            writeln!(
                stl_file,
                "      vertex {:.6} {:.6} {:.6}",
                vertex.x, vertex.y, vertex.z
            )?;
        }
        writeln!(stl_file, "    endloop")?;
        writeln!(stl_file, "  endfacet")?;
    }
    writeln!(stl_file, "endsolid Vec<Triangle>")?;

    Ok(())
}
