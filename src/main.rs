use std::num::NonZeroU64;

use num::complex::Complex64;

fn main() {
    brotten::solve(
        Complex64::new(-2., 1.),
        Complex64::new(1., -1.),
        NonZeroU64::new(1800).unwrap(),
        NonZeroU64::new(1200).unwrap(),
    )
}
