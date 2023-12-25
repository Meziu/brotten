use num::{complex::Complex64, Zero};
use std::{num::NonZeroU64, sync::Mutex};
use threadpool::ThreadPool;

const MAX_ITER: NonZeroU64 = unsafe { NonZeroU64::new_unchecked(255) };

pub fn mandelbrot_in_rect(
    start: Complex64,
    width: f64,
    height: f64,
    h_divs: NonZeroU64,
    v_divs: NonZeroU64,
    max_iter: NonZeroU64,
) -> Vec<(f64, f64, u64)> {
    let h_divs = h_divs.get();
    let v_divs = v_divs.get();

    let mut buf: Vec<(f64, f64, u64)> = Vec::with_capacity((h_divs*v_divs) as usize);

    let mut delta = Complex64::zero();

    delta.re = width / h_divs as f64;
    delta.im = height / v_divs as f64;

    for r in 0..=h_divs {
        for i in 0..=v_divs {
            let mut next = start;

            next.re += delta.re * r as f64;
            next.im -= delta.im * i as f64;

            let n = mandelbrot_diverges(next, max_iter);

            buf.push((next.re, next.im, n));
        }
    }

    buf
}

pub fn mandelbrot_diverges(c: Complex64, max_iter: NonZeroU64) -> u64 {
    let mut z = Complex64::zero();

    for i in 0..max_iter.into() {
        // Mandelbrot succession.
        z = z.powi(2) + c;

        // If the normal of the complex number is greater than 2, the succession diverges.
        if (z.re.powi(2) + z.im.powi(2)) >= 4. {
            return i;
        }
    }

    return max_iter.into();
}

/// Resolve the Mandelbrot set in the specified range based on the amounts of horizontal and vertical divisions.
///
/// This function automatically spreads the work between threads if possible.
pub fn solve(start: Complex64, end: Complex64, h_divs: NonZeroU64, v_divs: NonZeroU64) {
    let cores = num_cpus::get();

    // We divide the original rect by its width for the number of available cores.
    let subrect_width = (end.re - start.re) / cores as f64;
    let height: f64 = (end.im - start.im).abs();
    let h_divs_core = h_divs.get() / cores as u64;

    let pool = ThreadPool::new(cores);

    static RESULT: Mutex<Vec<Vec<(f64, f64, u64)>>> = Mutex::new(Vec::new());
    RESULT.lock().unwrap().resize(cores, Vec::new());

    for c in 0..cores {
        let final_vec = &RESULT;

        pool.execute(move || {
            let mut n_start = start;
            n_start.re += subrect_width * c as f64;

            let res = mandelbrot_in_rect(
                n_start,
                subrect_width,
                height,
                NonZeroU64::new(h_divs_core).unwrap(),
                NonZeroU64::new(v_divs.get()).unwrap(),
                MAX_ITER,
            );

            final_vec.lock().unwrap()[c] = res;
        });
    }

    pool.join();

    for v in RESULT.lock().unwrap().iter() {
        for (r, i, n) in v {
            println!("{r} {i} {n}");
        }
    }
}
