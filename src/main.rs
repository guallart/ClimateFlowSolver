mod stl;

fn main() {
    let tiff_path =
        r"/home/user/code/ClimateFlowSolver/elevation.tif";
    let grid = stl::Grid::from_tiff(tiff_path).expect("Failed reading tiff");

    let triangles = grid.triangulate();
    stl::write(triangles, "terrain.stl").expect("Could not write stl to file");

    let (north, south, west, east, sky) = grid.make_walls(100.0);
    stl::write(north, "north.stl").expect("Could not write stl to file");
    stl::write(south, "south.stl").expect("Could not write stl to file");
    stl::write(west, "east.stl").expect("Could not write stl to file");
    stl::write(east, "west.stl").expect("Could not write stl to file");
    stl::write(sky, "sky.stl").expect("Could not write stl to file");
}
