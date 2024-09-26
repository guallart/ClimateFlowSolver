use std::ops::{Add, Sub};

type FPrec = f64;

#[derive(Debug)]
struct Vector {
    x: FPrec,
    y: FPrec,
    z: FPrec,
}

impl Vector {
    fn from_xyz(x: FPrec, y: FPrec, z: FPrec) -> Vector {
        Vector { x, y, z }
    }

    fn from_slice(components: &[FPrec; 3]) -> Vector {
        Vector {
            x: components[0],
            y: components[1],
            z: components[2],
        }
    }

    fn cross_product(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn normalize(&self) -> Vector {
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn compute_normal(v1: &Vector, v2: &Vector, v3: &Vector) -> Vector {
        (v2 - v1).cross_product(&(v3 - v1)).normalize()
    }
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn main() {
    let v1 = Vector::from_xyz(1.0, 0.0, 0.0);
    let v2 = Vector::from_xyz(0.0, 1.0, 0.0);
    let v3 = Vector::from_xyz(0.0, 0.0, 1.0);

    let normal = Vector::compute_normal(&v1, &v2, &v3);
    println!("Normal vector: ({}, {}, {})", normal.x, normal.y, normal.z);
}
