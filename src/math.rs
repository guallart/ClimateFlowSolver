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

#[cfg(test)]
mod tests {
    use super::*;

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
}
