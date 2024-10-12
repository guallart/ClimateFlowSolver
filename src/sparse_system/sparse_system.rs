use crate::sparse_system::sparse_matrix::SparseMatrix;
use rayon::prelude::*;
use std::time::{Duration, Instant};

pub struct SparseSystem<'a> {
    coefficients: &'a SparseMatrix,
    column: &'a Vec<f64>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SolverResult {
    pub solution: Option<Vec<f64>>,
    pub converged: bool,
    pub diagonal_dominance: Option<bool>,
    pub iters: usize,
    pub tol: f64,
    pub max_iters_reached: bool,
    pub error: Option<f64>,
    pub message: String,
    pub elapsed_time: Option<Duration>,
}

impl<'a> SparseSystem<'a> {
    pub fn new(matrix: &'a SparseMatrix, column: &'a Vec<f64>) -> SparseSystem<'a> {
        SparseSystem {
            coefficients: matrix,
            column,
        }
    }

    pub fn error_sq(&self, x: &[f64]) -> f64 {
        // if self.use_iter {
        //     self.coefficients
        //         .dot_iter(&x)
        //         .zip(self.column.iter())
        //         .map(|(axi, bi)| (axi - bi).powi(2))
        //         .sum()
        // } else {
        self.coefficients
            .dot(&x)
            .unwrap()
            .iter()
            .zip(self.column.iter())
            .map(|(axi, bi)| (axi - bi).powi(2))
            .sum()
    }

    pub fn is_gauss_seidel_convergent(&self) -> bool {
        // diagonally dominant

        let mut diagonal = vec![0.0; self.coefficients.n_rows];
        let mut off_diagonal = vec![0.0; self.coefficients.n_rows];

        self.coefficients
            .diagonal_entries()
            .for_each(|(row, _col, value)| diagonal[*row] = value.abs());

        self.coefficients
            .off_diagonal_entries()
            .for_each(|(row, _col, value)| off_diagonal[*row] += value.abs());

        diagonal
            .iter()
            .zip(off_diagonal.iter())
            .all(|(d, od)| d >= od)
    }

    pub fn gauss_seidel_solve(&self, x0: &Vec<f64>, tol: f64, max_iters: usize) -> SolverResult {
        if x0.len() != self.coefficients.n_rows {
            return SolverResult {
                solution: None,
                converged: false,
                diagonal_dominance: None,
                iters: 0,
                tol,
                max_iters_reached: false,
                error: None,
                message: format!(
                    "Wrong dimensions [x0]={}    [A]={}x{}     [b]={}",
                    x0.len(),
                    self.coefficients.n_rows,
                    self.coefficients.n_cols,
                    self.column.len()
                ),
                elapsed_time: None,
            };
        }

        if !self.is_gauss_seidel_convergent() {
            return SolverResult {
                solution: None,
                converged: false,
                diagonal_dominance: Some(false),
                iters: 0,
                tol,
                max_iters_reached: false,
                error: None,
                message: "The coefficients matrix is not diagonally dominant".to_string(),
                elapsed_time: None,
            };
        }

        let start = Instant::now();
        let mut x = x0.clone();
        let mut sum_rows = vec![0.0; self.coefficients.n_rows];
        let diagonal: Vec<f64> = self.coefficients.diagonal_values().collect();

        for iter in 0..max_iters {
            sum_rows.fill(0.0);

            for (row, col, value) in self.coefficients.off_diagonal_entries() {
                sum_rows[*row] += (*value) * x[*col];
            }

            x.par_iter_mut().enumerate().for_each(|(i, x_val)| {
                *x_val = (self.column[i] - sum_rows[i]) / diagonal[i];
            });

            if self.error_sq(&x) < tol {
                return SolverResult {
                    error: Some(self.error_sq(&x)),
                    solution: Some(x),
                    converged: true,
                    diagonal_dominance: Some(true),
                    iters: iter + 1,
                    tol,
                    max_iters_reached: false,
                    message: format!("Converged in {} iterations", iter + 1),
                    elapsed_time: Some(start.elapsed()),
                };
            }
        }

        SolverResult {
            error: Some(self.error_sq(&x)),
            solution: Some(x),
            converged: true,
            diagonal_dominance: Some(true),
            iters: max_iters,
            tol,
            max_iters_reached: true,
            message: format!(
                "Converged in {} iterations (max iterations reached)",
                max_iters
            ),
            elapsed_time: Some(start.elapsed()),
        }
    }
}
