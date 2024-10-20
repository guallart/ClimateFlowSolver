mod boundary;
mod math;
mod mesh;
mod sparse_system;
#[macro_use]
mod benchmarking;

fn main() {
    let tiff_path = r"/home/user/code/ClimateFlowSolver/testing/elevation_cropped.tif";
    let vtk_path = r"/home/user/code/ClimateFlowSolver/testing/mesh.vtk";
    let terrain = boundary::Grid::from_tiff(tiff_path).unwrap();
    let z_values = math::linspace(terrain.z_min, terrain.z_max, 5);
    let mesh = mesh::mesher::Mesh::naive_mesh(&terrain, z_values);
    mesh.save_to_vtk(vtk_path).expect("Failefd at saving vtk");
}
