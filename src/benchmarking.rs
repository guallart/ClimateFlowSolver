#[macro_export]
macro_rules! bench {
    ($f:expr, $n_tries:expr) => {{
        use std::time::{Duration, Instant};

        fn mean_std(durations: Vec<Duration>) -> (Duration, Duration) {
            let n = durations.len();
            if n == 0 {
                return (Duration::new(0, 0), Duration::new(0, 0));
            }

            let mean_nanos = durations.iter().map(|d| d.as_nanos() as f64).sum::<f64>() / n as f64;
            let mean = Duration::from_nanos(mean_nanos as u64);

            let variance_nanos = durations
                .iter()
                .map(|d| {
                    let diff = d.as_nanos() as f64 - mean_nanos;
                    diff * diff
                })
                .sum::<f64>()
                / n as f64;

            let std_dev = Duration::from_nanos(variance_nanos.sqrt() as u64);

            (mean, std_dev)
        }

        let times: Vec<Duration> = (0..$n_tries)
            .map(|_| {
                let start = Instant::now();
                let _ = $f;
                start.elapsed()
            })
            .collect();

        let (mean, stddev) = mean_std(times);
        println!("{:?} +- {:?}", mean, stddev);
    }};
}
