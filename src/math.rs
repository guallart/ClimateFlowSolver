use num::traits::Float;

pub fn logspace<T: Float>(start: T, end: T, num: usize) -> impl Iterator<Item = T> {
    let lin_space: Vec<T> = (0..num)
        .map(|i| {
            let fraction = T::from(i).unwrap() / T::from(num - 1).unwrap();
            start + fraction * (end - start)
        })
        .collect();

    lin_space
        .into_iter()
        .map(|x| T::from(10.0).unwrap().powf(x))
}

fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
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
        let actual = logspace(0.0, 3.0, 4).collect::<Vec<f64>>();
        let expected = vec![1.0, 10.0, 100.0, 1000.0];
        assert_eq!(actual, expected);
    }
}
