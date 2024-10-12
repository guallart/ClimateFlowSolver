mod mesh;
mod sparse_system;
mod stl;
#[macro_use]
mod benchmarking;

use crate::sparse_system::sparse_matrix::SparseMatrix;

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

    // let file_path = "/home/user/code/ClimateFlowSolver/matrix_example.dat";
    // let a = SparseMatrix::load(file_path).unwrap();
    let a = SparseMatrix::random(10000, 10);
    let b = a.random_vec_like();

    bench!(a.dot_par(&b), 1000);
    bench!(a.dot(&b), 1000);
}
