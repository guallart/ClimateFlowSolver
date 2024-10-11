use crate::mesh::geometry::{Triangle, Vector};
use ndarray::{s, Array2};
use std::fs::File;
use std::io::Write;

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

    pub fn z(&self, row: usize, col: usize) -> f64 {
        self.elevations[[row, col]]
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

    pub fn from_tiff(path: &str) -> Result<Grid, gdal::errors::GdalError> {
        let dataset = gdal::Dataset::open(path)?;
        let band = dataset.rasterband(1)?;
        let (cols, rows) = band.size();

        let mut buf = vec![0.0; cols * rows];
        band.read_into_slice::<f64>(
            (0, 0),
            (cols, rows),
            (cols, rows),
            buf.as_mut_slice(),
            Some(gdal::raster::ResampleAlg::NearestNeighbour),
        )?;

        let mut elevations =
            Array2::from_shape_vec((rows, cols), buf).expect("Shape error at tiff to Array2");
        elevations.swap_axes(0, 1);
        let geo_transform = dataset.geo_transform()?;
        let x_min = geo_transform[0];
        let y_max = geo_transform[3];
        let x_res = geo_transform[1];
        let y_res = -geo_transform[5];
        let x_max = x_min + ((cols - 1) as f64) * x_res;
        let y_min = y_max - ((rows - 1) as f64) * y_res;

        Ok(Grid {
            elevations,
            x_min,
            y_min,
            x_max,
            y_max,
            x_res,
            y_res,
        })
    }
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
