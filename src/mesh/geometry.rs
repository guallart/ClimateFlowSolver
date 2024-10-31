#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn div(&self, d: f64) -> Vector {
        Vector {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub center: Vector,
    pub normal: Vector,
    pub vertices: [Vector; 3],
    pub area: f64,
}

#[derive(Clone)]
pub struct Quad {
    pub center: Vector,
    pub normal: Vector,
    pub vertices: [Vector; 4],
    pub area: f64,
}

impl Triangle {
    pub fn new(v1: &Vector, v2: &Vector, v3: &Vector) -> Triangle {
        let u = v2.sub(&v1);
        let v = v3.sub(&v1);
        let normal = u.cross(&v);
        let area = 0.5 * normal.mag();
        let center = v1.add(&v2).add(&v3).div(3.0);
        Triangle {
            center,
            vertices: [v1.clone(), v2.clone(), v3.clone()],
            normal,
            area,
        }
    }
}

impl Quad {
    pub fn new(v1: &Vector, v2: &Vector, v3: &Vector, v4: &Vector) -> Quad {
        let u = v2.sub(&v1);
        let v = v3.sub(&v1);
        let w = v4.sub(&v1);
        let normal1 = u.cross(&v);
        let normal2 = u.cross(&w);
        let area = 0.5 * (normal1.mag() + normal2.mag());
        let center = v1.add(&v2).add(&v3).add(&v4).div(4.0);
        Quad {
            center,
            vertices: [v1.clone(), v2.clone(), v3.clone(), v4.clone()],
            normal: normal1,
            area,
        }
    }
}

pub fn average_points(points: &[Vector]) -> Vector {
    if points.is_empty() {
        return Vector::new(0.0, 0.0, 0.0);
    }

    points
        .iter()
        .fold(Vector::new(0.0, 0.0, 0.0), |acc, point| acc.add(point))
        .div(points.len() as f64)
}
