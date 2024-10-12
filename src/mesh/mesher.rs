use crate::{
    mesh::geometry::{Quad, Triangle, Vector},
    stl::Grid,
};
use ndarray::s;
use ndarray::Array2;
use std::cmp::max;

pub enum WallType {
    Terrain,
    Sky,
    Inlet,
}

pub enum Wall {
    Triangle(Triangle),
    Quad(Quad),
}

pub struct Cell {
    id: usize,
    walls: Vec<Wall>,
    center: Vector,
    neighbours: Vec<usize>,
}

pub struct Mesh {
    cells: Vec<Cell>,
}

impl Wall {
    pub fn new_quad(v1: &Vector, v2: &Vector, v3: &Vector, v4: &Vector) -> Wall {
        Wall::Quad(Quad::new(v1, v2, v3, v4))
    }

    pub fn new_triangle(v1: &Vector, v2: &Vector, v3: &Vector, v4: &Vector) -> Wall {
        Wall::Triangle(Triangle::new(v1, v2, v3))
    }
}

impl Mesh {
    pub fn make_block_mesh(xs: Vec<f64>, ys: Vec<f64>, zs: Vec<f64>, terrain: Grid) -> Mesh {
        // Assume xs, ys and zs are sorted
        let nx = xs.len();
        let ny = ys.len();
        let nz = zs.len();
        let mut id_count = 0usize;

        let mut cells: Vec<Cell> = Vec::new();
        let mut depth = Array2::from_elem((ny, nx), 0);

        for i in 0..ny - 1 {
            for j in 0..nx - 1 {
                let max_height = [
                    terrain.z(i, j),
                    terrain.z(i + 1, j),
                    terrain.z(i, j + 1),
                    terrain.z(i + 1, j + 1),
                ]
                .into_iter()
                .reduce(f64::max)
                .unwrap();

                depth[(i, j)] = (0..nz)
                    .rev()
                    .take_while(|k| zs[k - 1] < max_height)
                    .last()
                    .unwrap()
                    + 1;
            }
        }

        // let walls:Array2<Vec<Wall>> = Array2::from_elem((ny, nx), );

        // for i in 0..nx - 1 {
        //     for j in 0..ny - 1 {
        //         walls
        //     }
        // }

        Mesh { cells: Vec::new() }
    }
}

// let v1 = Vector::new(xs[i], ys[j], zs[k]);
// let v2 = Vector::new(xs[i + 1], ys[j], zs[k]);
// let v3 = Vector::new(xs[i], ys[j + 1], zs[k]);
// let v4 = Vector::new(xs[i + 1], ys[j + 1], zs[k]);
// let v5 = Vector::new(xs[i], ys[j], zs[k - 1]);
// let v6 = Vector::new(xs[i + 1], ys[j], zs[k - 1]);
// let v7 = Vector::new(xs[i], ys[j + 1], zs[k - 1]);
// let v8 = Vector::new(xs[i + 1], ys[j + 1], zs[k - 1]);

// let w1 = Wall::new_quad(&v1, &v2, &v4, &v3);
// let w2 = Wall::new_quad(&v1, &v2, &v3, &v4);
