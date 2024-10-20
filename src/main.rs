mod boundary;
mod math;
mod mesh;
mod sparse_system;
#[macro_use]
mod benchmarking;

fn main() {
    let tiff_path = r"/home/user/code/ClimateFlowSolver/testing/elevation.tif";
    let stl_path = r"/home/user/code/ClimateFlowSolver/testing/boundary.stl";
    let vtk_path = r"/home/user/code/ClimateFlowSolver/testing/mesh.vtk";

    let terrain = boundary::Grid::from_tiff(tiff_path).unwrap();
    let max_height = terrain.z_max + 100.0;
    let min_height = terrain.z_max - 100.0;
    let z_values = math::linspace(min_height, max_height, 100);

    boundary::make_boundary_from_tiff(tiff_path, stl_path, max_height)
        .expect("Failed at saving boundary");
    let mesh = mesh::mesher::Mesh::naive_mesh(&terrain, z_values);
    mesh.save_to_vtk(vtk_path).expect("Failefd at saving vtk");
}
