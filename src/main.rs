mod stl;

use ndarray::Array2;
use stl::Grid;

fn main() {
    let tiff_path =
        r"C:\Users\javgua\Documents\Rust\ClimateFlowSolver\MDT25-ETRS89-H30-0411-4-COB2.tif";
    let grid = Grid::from_tiff(tiff_path);

    // let triangles = grid.triangulate();
    // stl::write(triangles, "terrain.stl").expect("Could not write stl to file");

    // let (north, south, west, east, sky) = grid.make_walls(2.0);
    // stl::write(north, "north.stl").expect("Could not write stl to file");
    // stl::write(south, "south.stl").expect("Could not write stl to file");
    // stl::write(west, "east.stl").expect("Could not write stl to file");
    // stl::write(east, "west.stl").expect("Could not write stl to file");
    // stl::write(sky, "sky.stl").expect("Could not write stl to file");
}
