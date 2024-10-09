pub mod sparse_matrix;
pub mod sparse_system;

use rand::Rng;
use sparse_matrix::SparseMatrix;
use sparse_system::SparseSystem;

fn random_vec(size: usize, low_limit: f64, high_limit: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(low_limit..high_limit))
        .collect::<Vec<f64>>()
}

fn test_system_solver() {
    println!("Generating system...");
    let n_rows = 1000000;
    let n_entries = ((n_rows as f64) * 1.5) as usize;
    let a = SparseMatrix::random(n_rows, n_entries);
    let b = random_vec(n_rows, -10.0, 10.0);
    let x0 = random_vec(n_rows, -1.0, 1.0);
    let tol = 1e-15;
    let max_iters = 100;

    println!("Solving system...");
    let s = SparseSystem::new(&a, &b);

    for _ in 0..5 {
        let res = s.gauss_seidel_solve(&x0, tol, max_iters);
        println!(
            "time = {:?}    converged = {}",
            res.elapsed_time.unwrap(),
            res.converged,
        );
    }
}
