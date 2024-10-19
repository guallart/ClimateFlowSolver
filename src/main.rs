mod math;
mod mesh;
mod sparse_system;
mod stl;
#[macro_use]
mod benchmarking;

use crate::math::logspace;
use crate::sparse_system::sparse_matrix::SparseMatrix;
use std::fs::File;
use std::io::{self, Write};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const THREAD_COUNT: usize = 4;
const GRID_SIZE: usize = THREAD_COUNT * 10000;
const CELLS_PER_THREAD: usize = GRID_SIZE / THREAD_COUNT;

static mut GRID: [f64; GRID_SIZE] = [0.0; GRID_SIZE];

fn main() {
    // let tiff_path =
    //     r"/home/user/code/ClimateFlowSolver/elevation_cropped.tif";
    // let grid = stl::Grid::from_tiff(tiff_path).expect("Failed reading tiff");

    // let triangles = grid.triangulate();
    // stl::write(triangles, "terrain.stl").expect("Could not write stl to file");

    // let (north, south, west, east, sky) = grid.make_walls(100.0);
    // stl::write(north, "north.stl").expect("Could not write stl to file");
    // stl::write(south, "south.stl").expect("Could not write stl to file");
    // stl::write(west, "east.stl").expect("Could not write stl to file");
    // stl::write(east, "west.stl").expect("Could not write stl to file");
    // stl::write(sky, "sky.stl").expect("Could not write stl to file");

    //-------------------------------------------------------------------------------------

    // let file_path = "/home/user/code/ClimateFlowSolver/matrix_example.dat";
    // let a = SparseMatrix::load(file_path).unwrap();

    // let filename = r"~/code/ClimateFlowSolver/dot_times.dat";
    // let mut file = File::create(filename).expect("<sfsfg");

    // for entries_count in logspace(1.0, 5.0, 100) {
    //     let a = SparseMatrix::random(10000, 50, true);
    //     let b = a.random_vec_like();

    // }

    // diff_time!(a.dot_par(&b), a.dot(&b), 1000);

    //-------------------------------------------------------------------------------------

    let grid = Arc::new(Mutex::new(vec![0.0; GRID_SIZE]));

    diff_time!(arc_mutex_grid(&grid), static_grid(), 100);

    unsafe {
        let data_ref = Arc::clone(&grid);
        let vec = data_ref.lock().unwrap();

        let err = GRID
            .iter()
            .zip(vec.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<f64>();

        println!(" abs err = {}", err);
    }
}

fn arc_mutex_grid(grid: &Arc<Mutex<Vec<f64>>>) {
    let mut handles = Vec::new();

    for i in 0..THREAD_COUNT {
        let grid_clone = Arc::clone(grid);
        let handle = thread::spawn(move || {
            for j in 0..CELLS_PER_THREAD {
                let mut grid = grid_clone.lock().unwrap();
                grid[i * CELLS_PER_THREAD + j] = 0.0;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn static_grid() {
    let mut handles = Vec::new();

    for i in 0..THREAD_COUNT {
        let handle = thread::spawn(move || unsafe {
            for j in 0..CELLS_PER_THREAD {
                GRID[i * CELLS_PER_THREAD + j] = 0.0;
            }
        });

        handles.push(handle);
    }

    for h in handles.into_iter() {
        h.join().unwrap();
    }
}
