use itertools::izip;
use rand::Rng;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

type SparseEntry = (usize, usize, f64);

#[derive(Clone)]
pub struct SparseMatrix {
    pub entries: Vec<SparseEntry>,
    pub n_rows: usize,
    pub n_cols: usize,
    pub row_indices: Vec<Option<(usize, usize)>>,
}

impl SparseMatrix {
    #[allow(dead_code)]
    pub fn new(estimated_entries_count: usize) -> Self {
        SparseMatrix {
            entries: Vec::with_capacity(estimated_entries_count),
            n_rows: 0,
            n_cols: 0,
            row_indices: Vec::new(),
        }
    }

    pub fn compute_size(&mut self) {
        for (row, col, _val) in self.entries.iter() {
            self.n_rows = self.n_rows.max(row + 1);
            self.n_cols = self.n_cols.max(col + 1);
        }
    }

    pub fn sort_entries(&mut self) {
        self.entries.sort_by(|a, b| {
            let cmp = a.0.cmp(&b.0);
            if cmp == std::cmp::Ordering::Equal {
                a.1.cmp(&b.1)
            } else {
                cmp
            }
        });
    }

    pub fn compute_row_indices(&mut self) {
        // Assume entries are sorted
        self.row_indices = vec![None; self.n_rows];

        let mut current_row = 0;
        let mut start_index = 0;

        for (i, &(row, _, _)) in self.entries.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if current_row != row {
                self.row_indices[current_row] = Some((start_index, i - 1));
                current_row = row;
                start_index = i;
            }
        }

        self.row_indices[current_row] = Some((start_index, self.entries.len() - 1));
    }

    pub fn preprocess(&mut self) {
        self.compute_size();
        self.sort_entries();
        self.compute_row_indices();
    }

    pub fn random(n_rows: usize, n_entries: usize, force_diagonal: bool) -> SparseMatrix {
        let mut rng = rand::thread_rng();

        let mut indices: Vec<(usize, usize)> = (0..n_entries)
            .map(|_| (rng.gen_range(0..n_rows), rng.gen_range(0..n_rows)))
            .filter(|(r, c)| *r != *c)
            .collect();

        if force_diagonal {
            for i in 0..n_rows {
                indices.push((i, i));
            }
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

    pub fn random_vec_like(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        (0..self.n_cols)
            .map(|_| rng.gen::<f64>() * 2.0 - 1.0)
            .collect()
    }

    pub fn from_vecs(rows: &Vec<usize>, cols: &Vec<usize>, values: &Vec<f64>) -> SparseMatrix {
        let entries = izip!(rows, cols, values)
            .map(|(row, col, val)| (*row, *col, *val))
            .collect();

        let mut matrix = SparseMatrix {
            entries,
            n_rows: 0,
            n_cols: 0,
            row_indices: Vec::new(),
        };

        matrix.preprocess();
        matrix
    }

    pub fn dot_par(&self, x: &[f64]) -> Result<Vec<f64>, String> {
        if self.n_cols != x.len() {
            return Err(format!(
                "Cannot multiply a {}x{} matrix with a {}x1 vector",
                self.n_rows,
                self.n_cols,
                x.len()
            ));
        }

        let b = self
            .row_indices
            .par_iter()
            .map(|maybe_range| match maybe_range {
                Some((a, b)) => (*a..=*b)
                    .map(|i| self.entries[i].2 * x[self.entries[i].1])
                    .sum(),
                None => 0.0,
            })
            .collect();

        Ok(b)
    }
    pub fn dot(&self, x: &[f64]) -> Result<Vec<f64>, String> {
        if self.n_cols != x.len() {
            return Err(format!(
                "Cannot multiply a {}x{} matrix with a {}x1 vector",
                self.n_rows,
                self.n_cols,
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

    pub fn save(&self, file_path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        for (row, col, val) in self.entries.iter() {
            writeln!(writer, "{} {} {}", row, col, val)?;
        }

        writer.flush()?;

        Ok(())
    }

    pub fn load(file_path: impl AsRef<Path>) -> Result<SparseMatrix, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 3 {
                let row: usize = parts[0].parse()?;
                let col: usize = parts[1].parse()?;
                let val: f64 = parts[2].parse()?;
                entries.push((row, col, val));
            }
        }

        let mut matrix = SparseMatrix {
            entries,
            n_cols: 0,
            n_rows: 0,
            row_indices: Vec::new(),
        };

        matrix.preprocess();
        Ok(matrix)
    }
}

impl fmt::Display for SparseMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n")?;
        for (row, col, value) in self.entries.iter() {
            write!(f, "   ({}, {}): {}\n", *row, *col, *value)?;
        }
        write!(f, "] n_rows={:?}", self.n_rows)?;
        Ok(())
    }
}
