use std::fs::File;
use std::io::Write;

#[derive(Clone)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

struct Triangle {
    normal: Vector,
    vertices: [Vector; 3],
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }
}

fn compute_normal(v1: &Vector, v2: &Vector, v3: &Vector) -> Vector {
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

fn make_stl(vertices: Vec<Vector>) -> Vec<Triangle> {
    // TODO: change this function to create stl from grid and limit walls
    let mut triangles = Vec::new();

    for chunk in vertices.chunks(3) {
        if chunk.len() != 3 {
            continue;
        }

        let v1 = chunk[0].clone();
        let v2 = chunk[1].clone();
        let v3 = chunk[2].clone();

        let normal = compute_normal(&v1, &v2, &v3);

        let triangle = Triangle {
            normal,
            vertices: [v1, v2, v3],
        };

        triangles.push(triangle);
    }
    triangles
}

fn write_stl(triangles: Vec<Triangle>, file_name: &str) -> Result<(), std::io::Error> {
    let mut stl_file = File::create(file_name)?;
    writeln!(stl_file, "solid mesh")?;
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
    writeln!(stl_file, "endsolid mesh")?;

    Ok(())
}

fn main() {
    let v1 = Vector::new(1.0, 0.0, 0.0);
    let v5 = Vector::new(1.0, 0.0, 0.0);
    let v2 = Vector::new(0.0, 1.0, 0.0);
    let v6 = Vector::new(0.0, 1.0, 0.0);
    let v3 = Vector::new(0.0, 0.0, 1.0);
    let v4 = Vector::new(0.0, 0.0, 2.0);

    let vertices = vec![v1, v2, v3, v5, v4, v6];
    let stl = make_stl(vertices);
    write_stl(stl, "test.stl").expect("Could not write stl to file");
}
