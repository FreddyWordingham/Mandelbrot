use crate::complex::Complex;
use pyo3::prelude::*;

/// Determine the number of iterations required to escape a point.
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

/// Determine the average number of iterations required to escape a region.
#[pyfunction]
pub fn multi_sample(c: Complex, max_iter: i32, super_samples: i32, delta: f64) -> i32 {
    let start = c + Complex::new(
        delta * (1 - super_samples) as f64,
        delta * (1 - super_samples) as f64,
    );
    let mut total = 0;
    for i in 0..super_samples {
        let re = delta * (i as f64);
        for j in 0..super_samples {
            let im = delta * (j as f64);
            total += sample(start + Complex::new(re, im), max_iter);
        }
    }
    total
}

fn sample_area() {}

fn data_to_col() {}

fn col_to_image() {}

pub fn image(centre: Complex, scale: f64, res: [usize; 2], super_samples: i32, max_iter: i32) {}
