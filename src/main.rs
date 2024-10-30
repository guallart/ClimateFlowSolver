use mesh::mesher;

mod boundary;
mod math;
mod mesh;
mod sparse_system;
#[macro_use]
mod benchmarking;

fn main() {
    let testing_dir = std::env::current_dir().unwrap().join("testing");
    let tiff_path = testing_dir.join("elevation.tif");
    let stl_path = testing_dir.join("boundary.stl");
    let vtk_path = testing_dir.join("mesh.vtk");

    let terrain = boundary::Grid::from_tiff(tiff_path).unwrap();
    let height_amp = terrain.z_max - terrain.z_min;
    let max_height = terrain.z_max + height_amp * 0.5;
    let min_height = terrain.z_min - height_amp * 0.1;
    let z_values = math::linspace(min_height, max_height, 5);

    let initial_conditions = mesher::InitialPhysics {
        z_ref: 500.0,
        speed_ref: 6.0,
        density_ref: 1.225,
        direction: 0.0,
        shear: 0.2,
        temperature: 300.0,
    };

    terrain
        .make_boundary(stl_path, height_amp * 0.5)
        .expect("Failed at saving boundary");

    let mut mesh = mesh::mesher::Mesh::naive_mesh(&terrain, z_values);
    mesh.define_initial_and_boundary_conditions(initial_conditions);
    mesh.save_to_vtk(vtk_path).expect("Failed at saving vtk");
}
