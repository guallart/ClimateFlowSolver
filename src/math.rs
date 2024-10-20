#[allow(unused)]
pub fn logspace(start: f64, end: f64, num: usize) -> Vec<f64> {
    let lin_space: Vec<f64> = (0..num)
        .map(|i| {
            let fraction = i as f64 / ((num - 1) as f64);
            start + fraction * (end - start)
        })
        .collect();

    lin_space.into_iter().map(|x| (10 as f64).powf(x)).collect()
}

#[allow(unused)]
pub fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    if num < 2 {
        return vec![start];
    }

    let step = (stop - start) / (num - 1) as f64;
    let mut result = Vec::with_capacity(num);

    for i in 0..num - 1 {
        let value = start + step * i as f64;
        result.push(value);
    }

    result.push(stop);
    result
}

#[allow(unused)]
struct Interpolator {
    x_vals: Vec<f64>,
    y_vals: Vec<f64>,
}

#[allow(unused)]
impl Interpolator {
    pub fn new(x_vals: Vec<f64>, y_vals: Vec<f64>) -> Result<Interpolator, String> {
        if !x_vals.windows(2).all(|w| w[0] <= w[1]) {
            return Err("x values not ordered".to_string());
        }
        Ok(Interpolator { x_vals, y_vals })
    }

    pub fn interp(&self, x: f64) -> f64 {
        if x < self.x_vals[0] {
            return self.y_vals[0];
        }

        let idx = self.x_vals.len() - 1;
        if x > self.x_vals[idx] {
            return self.y_vals[idx];
        }

        for (xi, yi) in self.x_vals.iter().zip(self.y_vals.iter()) {
            if x == *xi {
                return *yi;
            }
        }

        let i = self.x_vals.partition_point(|&val| val <= x);
        let x0 = self.x_vals[i - 1];
        let x1 = self.x_vals[i];
        let y0 = self.y_vals[i - 1];
        let y1 = self.y_vals[i];

        y0 + (x - x0) * (y1 - y0) / (x1 - x0)
    }

    pub fn interp1d(&self, x: Vec<f64>) -> Vec<f64> {
        x.into_iter().map(|xi| self.interp(xi)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_linspace() {
        let actual = linspace(1.0, 7.0, 4);
        let expected = vec![1.0, 3.0, 5.0, 7.0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_logspace() {
        let actual = logspace(0.0, 3.0, 4);
        let expected = vec![1.0, 10.0, 100.0, 1000.0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_interp() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 3.0, 5.0];
        let interp = Interpolator::new(x, y).unwrap();
        assert_relative_eq!(interp.interp(1.5), 2.0, epsilon = 1e-6);
        assert_relative_eq!(interp.interp(2.5), 4.0, epsilon = 1e-6);
        assert_relative_eq!(interp.interp(0.5), 1.0, epsilon = 1e-6);
        assert_relative_eq!(interp.interp(3.5), 5.0, epsilon = 1e-6);
        assert_relative_eq!(interp.interp(1.0), 1.0, epsilon = 1e-6);
        assert_relative_eq!(interp.interp(2.0), 3.0, epsilon = 1e-6);
        assert_relative_eq!(interp.interp(3.0), 5.0, epsilon = 1e-6);

        let x_test = vec![1.5, 2.5];
        let y_test = interp.interp1d(x_test);
        assert_relative_eq!(y_test[0], 2.0, epsilon = 1e-6);
        assert_relative_eq!(y_test[1], 4.0, epsilon = 1e-6);
    }
}
