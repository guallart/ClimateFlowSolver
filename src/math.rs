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
