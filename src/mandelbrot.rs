use crate::complex::Complex;
use pyo3::prelude::*;

/// Determine the number of iterations required to escape.
#[pyfunction]
pub fn sample(c: Complex, max_iter: i32) -> i32 {
    let mut z = Complex::zero();
    for n in 0..max_iter {
        z = z * z + c.clone();
        if z.norm_squared() > 4.0 {
            return n;
        }
    }
    return max_iter;
}
