mod stl;

use ndarray::Array2;

fn main() {
    let data = Array2::from_shape_vec(
        (6, 8),
        vec![
            0.4176358598742931,
            0.4545517045414713,
            0.328033802464857,
            0.24510358343499028,
            0.17086728800245676,
            0.22084883958167956,
            0.9769414259176685,
            0.32636739942180193,
            0.8778783125108963,
            0.4496043875649999,
            0.6814220589578439,
            0.03012766680110568,
            0.3631645318122748,
            0.3862029849265707,
            0.9294344568440851,
            0.5953782666645141,
            0.1211362874829568,
            0.8357689472469375,
            0.19800880240978236,
            0.5224051213215922,
            0.5927041954543428,
            0.9593558518996702,
            0.2773746333571905,
            0.796183618868569,
            0.05003216167409075,
            0.8898108913982086,
            0.8241386826438135,
            0.4110409594688641,
            0.9142240673937159,
            0.5776898731316458,
            0.011323981682928363,
            0.8074568356212808,
            0.8739076253737043,
            0.08811322971688307,
            0.7611076076957661,
            0.1662999556203798,
            0.5897888480252637,
            0.5987962821869194,
            0.8473257138941037,
            0.009600135263067355,
            0.5131200670997168,
            0.9949134368115533,
            0.5897888480252637,
            0.5987962821869194,
            0.8473257138941037,
            0.009600135263067355,
            0.5131200670997168,
            0.9949134368115533,
        ],
    )
    .unwrap();

    let (nrows, ncols) = data.dim();

    let grid = stl::Grid {
        points: data,
        x_min: 0.0,
        y_min: 0.0,
        x_res: 1.1,
        y_res: 1.2,
        x_max: 0.0 + 1.1 * ((ncols - 1) as f64),
        y_max: 0.0 + 1.2 * ((nrows - 1) as f64),
    };

    let triangles = grid.triangulate();
    stl::write(triangles, "mesh.stl").expect("Could not write stl to file");

    let (north, south, west, east, sky) = grid.make_walls(2.0);
    stl::write(north, "north.stl").expect("Could not write stl to file");
    stl::write(south, "south.stl").expect("Could not write stl to file");
    stl::write(west, "east.stl").expect("Could not write stl to file");
    stl::write(east, "west.stl").expect("Could not write stl to file");
    stl::write(sky, "sky.stl").expect("Could not write stl to file");
}
