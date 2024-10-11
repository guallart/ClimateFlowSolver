use core::f64;
use itertools::izip;
use rand::Rng;
use std::fmt;

type SparseEntry = (usize, usize, f64);

#[derive(Clone)]
pub struct SparseMatrix {
    pub entries: Vec<SparseEntry>,
    pub n_rows: usize,
    pub n_cols: usize,
}

impl SparseMatrix {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SparseMatrix {
            entries: Vec::new(),
            n_rows: 0,
            n_cols: 0,
        }
    }

    pub fn random(n_rows: usize, n_entries: usize) -> SparseMatrix {
        let mut rng = rand::thread_rng();

        let mut indices: Vec<(usize, usize)> = (0..n_entries)
            .map(|_| (rng.gen_range(0..n_rows), rng.gen_range(0..n_rows)))
            .filter(|(r, c)| *r != *c)
            .collect();

        for i in 0..n_rows {
            indices.push((i, i));
        }

        let values: Vec<f64> = indices
            .iter()
            .map(|(row, col)| {
                if *row == *col {
                    rng.gen_range(5000.0..10000.0)
                } else {
                    rng.gen_range(-10.0..10.0)
                }
            })
            .collect();

        let (rows, cols) = indices.into_iter().unzip();

        SparseMatrix::from_vecs(&rows, &cols, &values)
    }

    pub fn from_vecs(rows: &Vec<usize>, cols: &Vec<usize>, values: &Vec<f64>) -> SparseMatrix {
        let mut matrix = SparseMatrix {
            entries: Vec::with_capacity(rows.len()),
            n_rows: 0,
            n_cols: 0,
        };
        matrix.add_from_vecs(rows, cols, values);
        matrix.sort_entries();
        matrix
    }

    pub fn add_entry(&mut self, row: usize, col: usize, value: f64) {
        self.entries.push((row, col, value));

        if self.n_rows == 0 {
            self.n_rows = row + 1;
            return;
        }

        if row + 1 > self.n_rows {
            self.n_rows = row + 1;
        }

        if col + 1 > self.n_cols {
            self.n_cols = col + 1;
        }
    }

    pub fn sort_entries(&mut self) {
        self.entries.sort_by(|ent1, ent2| ent1.0.cmp(&ent2.0));
    }

    pub fn add_from_vecs(&mut self, rows: &Vec<usize>, cols: &Vec<usize>, values: &Vec<f64>) {
        for (row, col, value) in izip!(rows, cols, values) {
            self.add_entry(*row, *col, *value);
        }
    }

    pub fn dot(&self, x: &[f64]) -> Result<Vec<f64>, String> {
        if self.n_rows != x.len() {
            return Err(format!(
                "Cannot multiply a {}x{} matrix with a {}x1 vector",
                self.n_rows,
                self.n_rows,
                x.len()
            ));
        }

        // A x = b
        let mut b: Vec<f64> = vec![0.0; self.n_rows];

        for (row, col, value) in self.entries.iter() {
            b[*row] += x[*col] * value;
        }

        Ok(b)
    }

    // pub fn dot_iter<'a>(&'a self, x: &'a [f64]) -> impl Iterator<Item = f64> + 'a {
    //     // Assume entries are sorted by row number
    //     let mut entry_iter = self.entries.iter().peekable();
    //     let mut current_row = 0;

    //     std::iter::from_fn(move || {
    //         if current_row >= self.n_rows {
    //             return None;
    //         }

    //         let mut sum = 0.0;
    //         while let Some(&(row, col, value)) = entry_iter.peek() {
    //             if *row == current_row {
    //                 sum += x[*col] * value;
    //                 entry_iter.next();
    //             } else {
    //                 break;
    //             }
    //         }

    //         current_row += 1;
    //         Some(sum)
    //     })
    // }

    pub fn diagonal_entries(&self) -> impl Iterator<Item = &SparseEntry> {
        self.entries
            .iter()
            .filter(|(row, col, _value)| *row == *col)
    }

    pub fn diagonal_values(&self) -> impl Iterator<Item = f64> + '_ {
        self.entries
            .iter()
            .filter(|(row, col, _value)| *row == *col)
            .map(|(_row, _col, value)| *value)
    }

    pub fn off_diagonal_entries(&self) -> impl Iterator<Item = &SparseEntry> {
        self.entries
            .iter()
            .filter(|(row, col, _value)| *row != *col)
    }
}

impl fmt::Display for SparseMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n")?;
        for (row, col, value) in self.entries.iter() {
            write!(f, "   ({}, {}): {}\n", *row, *col, *value)?;
        }
        write!(f, "] size={:?}", self.n_rows)?;
        Ok(())
    }
}
