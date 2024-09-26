mod stl;

fn main() {
    let v1 = stl::Vector::new(1.0, 0.0, 0.0);
    let v5 = stl::Vector::new(1.0, 0.0, 0.0);
    let v2 = stl::Vector::new(0.0, 1.0, 0.0);
    let v6 = stl::Vector::new(0.0, 1.0, 0.0);
    let v3 = stl::Vector::new(0.0, 0.0, 1.0);
    let v4 = stl::Vector::new(0.0, 0.0, 2.0);

    let vertices = vec![v1, v2, v3, v5, v4, v6];
    let stl: Vec<stl::Triangle> = stl::from_vertices(vertices);
    stl::write(stl, "test.stl").expect("Could not write stl to file");
}
