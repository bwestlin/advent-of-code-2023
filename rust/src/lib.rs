extern crate time;

use std::time::*;

pub fn measure<F, S, T>(f: F) -> Result<S, T>
where
    F: Fn() -> Result<S, T>,
{
    let start = Instant::now();
    let mut _times = 100;

    #[cfg(not(feature = "timeit"))]
    let res = f()?;

    #[cfg(feature = "timeit")]
    let mut res = f()?;

    #[cfg(feature = "timeit")]
    {
        let dur = start.elapsed();
        if dur.as_nanos() > 500_000_000 {
            _times /= 10;
        } else if dur.as_nanos() < 500_000 {
            _times *= 10;
        }
        for _ in 0..(_times - 1) {
            res = f()?;
        }
    }

    let dur = start.elapsed();

    #[cfg(feature = "timeit")]
    {
        println!(
            "It took: {}ms on average for {} times",
            (dur.as_nanos() / _times as u128) as f64 / 1_000_000.0,
            _times
        );
    }
    #[cfg(not(feature = "timeit"))]
    {
        println!("It took: {}ms", dur.as_nanos() as f64 / 1_000_000.0);
    }
    Ok(res)
}
