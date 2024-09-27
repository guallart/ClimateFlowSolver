use ndarray::{s, Array2};
use std::io::Write;
use std::{clone, fs::File};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }
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
    pub elevations: Array2<f64>,
    pub x_min: f64,
    pub y_min: f64,
    pub x_max: f64,
    pub y_max: f64,
    pub x_res: f64,
    pub y_res: f64,
}

impl Grid {
    pub fn x(&self, col: usize) -> f64 {
        self.x_min + self.x_res * (col as f64)
    }

    pub fn y(&self, row: usize) -> f64 {
        self.y_min + self.y_res * (row as f64)
    }

    pub fn xyz(&self, col: usize, row: usize) -> Vector {
        Vector {
            x: self.x(col),
            y: self.y(row),
            z: self.elevations[[col, row]],
        }
    }

    pub fn triangulate(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let (cols, rows) = self.elevations.dim();

        for c in 0..cols - 1 {
            for r in 0..rows - 1 {
                let v1 = self.xyz(c, r);
                let v2 = self.xyz(c + 1, r);
                let v3 = self.xyz(c, r + 1);
                let v4 = self.xyz(c + 1, r + 1);

                triangles.push(Triangle::new(&v1, &v2, &v3));
                triangles.push(Triangle::new(&v2, &v4, &v3));
            }
        }

        triangles
    }

    fn make_single_wall(limit: &Vec<Vector>, z_upper: f64) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = Vec::new();

        for window in limit.windows(2) {
            let (v1, v2) = (window[0], window[1]);
            let v3 = Vector::new(v1.x, v1.y, z_upper);
            let v4 = Vector::new(v2.x, v2.y, z_upper);
            triangles.push(Triangle::new(&v1, &v2, &v3));
            triangles.push(Triangle::new(&v2, &v4, &v3));
        }

        triangles
    }

    pub fn make_walls(
        &self,
        max_heigh: f64,
    ) -> (
        Vec<Triangle>,
        Vec<Triangle>,
        Vec<Triangle>,
        Vec<Triangle>,
        Vec<Triangle>,
    ) {
        let height = max_heigh
            + self
                .elevations
                .iter()
                .filter(|&x| !x.is_nan())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

        let points_north = self
            .elevations
            .slice(s![.., -1])
            .into_iter()
            .enumerate()
            .map(|(i, z)| Vector::new(self.x(i), self.y_max, *z))
            .collect::<Vec<Vector>>();

        let points_south = self
            .elevations
            .slice(s![.., 0])
            .into_iter()
            .enumerate()
            .map(|(i, z)| Vector::new(self.x(i), self.y_min, *z))
            .collect::<Vec<Vector>>();

        let points_west = self
            .elevations
            .slice(s![0, ..])
            .into_iter()
            .enumerate()
            .map(|(j, z)| Vector::new(self.x_min, self.y(j), *z))
            .collect::<Vec<Vector>>();

        let points_east = self
            .elevations
            .slice(s![-1, ..])
            .into_iter()
            .enumerate()
            .map(|(j, z)| Vector::new(self.x_max, self.y(j), *z))
            .collect::<Vec<Vector>>();

        let north = Grid::make_single_wall(&points_north, height);
        let south = Grid::make_single_wall(&points_south, height);
        let west = Grid::make_single_wall(&points_west, height);
        let east = Grid::make_single_wall(&points_east, height);

        let nw = Vector::new(self.x_min, self.y_max, height);
        let ne = Vector::new(self.x_max, self.y_max, height);
        let sw = Vector::new(self.x_min, self.y_min, height);
        let se = Vector::new(self.x_max, self.y_min, height);

        let mut sky: Vec<Triangle> = Vec::new();
        sky.push(Triangle::new(&nw, &se, &sw));
        sky.push(Triangle::new(&nw, &ne, &se));

        (north, south, west, east, sky)
    }

    pub fn from_tiff(path_tiff: &str) -> Result<Grid, Box<dyn std::error::Error>> {
        todo!();
        // let file_tiff = File::open(path_tiff)?;
        // let mut decoder = Decoder::new(file_tiff)?;
        // let (cols, rows) = decoder.dimensions()?;
        // let image_data = decoder.read_image()?;

        // let data = match image_data {
        //     DecodingResult::F32(data) => data.iter().map(|x| *x as f64).collect(),
        //     DecodingResult::F64(data) => data,
        //     _ => panic!("Unsoported tiff format"),
        // };

        // let z = decoder.get_tag_f64(tiff::tags::Tag::MinSampleValue)?;
        // println!("{}", z);

        // let mut x_coords: Vec<f64> = Vec::new();
        // let mut y_coords: Vec<f64> = Vec::new();
        // let mut z_coords: Vec<f64> = Vec::new();

        // data.chunks(3).for_each(|chunk| {
        //     let (x, y, z) = (chunk[0], chunk[1], chunk[2]);
        //     x_coords.push(x);
        //     y_coords.push(y);
        //     z_coords.push(z);
        // });

        // let elevations = Array2::from_shape_vec((rows as usize, cols as usize), z_coords)?;
        // let (x_min, x_max, x_res) = min_max_res(&x_coords);
        // let (y_min, y_max, y_res) = min_max_res(&y_coords);

        // let grid = Grid {
        //     elevations,
        //     x_min,
        //     y_min,
        //     x_max,
        //     y_max,
        //     x_res,
        //     y_res,
        // };

        // Ok(grid)
    }
}

fn min_max_res(values: &[f64]) -> (f64, f64, f64) {
    let max_value = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_value = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    let spacing = if values.len() > 1 {
        values[1] - values[0]
    } else {
        0.0
    };

    (min_value, max_value, spacing)
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
