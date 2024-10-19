#[macro_export]
macro_rules! timeit {
    ($f:expr, $n_tries:expr) => {{
        use std::time::{Duration, Instant};

        let durations: Vec<f64> = (0..$n_tries)
            .map(|_| {
                let start = Instant::now();
                let _ = $f;
                start.elapsed().as_nanos() as f64
            })
            .collect();

        let mean_nanos = durations.iter().sum::<f64>() / $n_tries as f64;
        let mean = Duration::from_nanos(mean_nanos as u64);

        let variance_nanos = durations
            .iter()
            .map(|d| (d - mean_nanos).powi(2))
            .sum::<f64>()
            / $n_tries as f64;

        let std_dev = Duration::from_nanos(variance_nanos.sqrt() as u64);
        (mean, std_dev)
    }};
}

#[macro_export]
macro_rules! logtime {
    ($f:expr, $n_tries:expr) => {
        let (mean, std_dev) = bench!($f, $n_tries);
        println!("{:?} +- {:?}", mean, std_dev);
    };
}

#[macro_export]
macro_rules! diff_time {
    ($f:expr, $g:expr, $n_tries:expr) => {
        let (mean1, std_dev1) = timeit!($f, $n_tries);
        let (mean2, std_dev2) = timeit!($f, $n_tries);
        let mean1_nanos = mean1.as_nanos() as f64;
        let mean2_nanos = mean2.as_nanos() as f64;
        let std_dev1_nanos = std_dev1.as_nanos() as f64;
        let std_dev2_nanos = std_dev2.as_nanos() as f64;
        let n = $n_tries as f64;
        let sp = ((n - 1.0) * (std_dev1_nanos * std_dev1_nanos + std_dev2_nanos * std_dev2_nanos)
            / (2.0 * n - 2.0))
            .sqrt();
        let t = (mean1_nanos - mean2_nanos) / (sp * (2.0 / n).sqrt());

        let t_table = vec![
            (1, 6.314),
            (2, 2.920),
            (3, 2.353),
            (4, 2.132),
            (5, 2.015),
            (6, 1.943),
            (7, 1.895),
            (8, 1.860),
            (9, 1.833),
            (10, 1.812),
            (11, 1.796),
            (12, 1.782),
            (13, 1.771),
            (14, 1.761),
            (15, 1.753),
            (16, 1.746),
            (17, 1.740),
            (18, 1.734),
            (19, 1.729),
            (20, 1.725),
            (21, 1.721),
            (22, 1.717),
            (23, 1.714),
            (24, 1.711),
            (25, 1.708),
            (26, 1.706),
            (27, 1.703),
            (28, 1.701),
            (29, 1.699),
            (30, 1.697),
            (40, 1.684),
            (60, 1.671),
            (80, 1.664),
            (100, 1.660),
            (1000, 1.646),
        ];

        let deg_free = n - 2.0;
        let t_limit = t_table
            .iter()
            .min_by_key(|&&(val, _)| (val as i32 - deg_free as i32).abs())
            .map(|&(_, f)| f)
            .unwrap();

        if t > t_limit {
            println!("different performance       P(T<{}) < 0.05", t_limit);
        } else {
            println!("NOT different performance   P(T<{}) > 0.05", t_limit);
        }

        println!(
            "  -  {:<25}   {:?} +- {:?}",
            std::stringify!($f),
            mean1,
            std_dev1
        );
        println!(
            "  -  {:<25}   {:?} +- {:?}",
            std::stringify!($g),
            mean2,
            std_dev2
        );
    };
}
