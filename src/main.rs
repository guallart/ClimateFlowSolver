mod boundary;
mod math;
mod mesh;
mod sparse_system;
#[macro_use]
mod benchmarking;

use std::path::Path;

fn main() {
    let input_folder = Path::new(r"~/code/ClimateFlowSolver");
    let tiff_path = input_folder.join("elevation.tif");
    let stl_path = input_folder.join("boundary.stl");
    let vtk_path = input_folder.join("boundary.stl");

    let terrain = boundary::Grid::from_tiff(tiff_path).unwrap();
    let max_height = terrain.z_max + 100.0;
    let min_height = terrain.z_max - 100.0;
    let z_values = math::linspace(min_height, max_height, 100);

    boundary::make_boundary_from_tiff(tiff_path, stl_path, max_height)
        .expect("Failed at saving boundary");
    let mesh = mesh::mesher::Mesh::naive_mesh(&terrain, z_values);
    mesh.save_to_vtk(vtk_path).expect("Failefd at saving vtk");
}
