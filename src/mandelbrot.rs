use crate::complex::Complex;
use ndarray::Array2;
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
pub fn multi_sample(c: Complex, max_iter: i32, super_samples: i32, epsilon: f64) -> i32 {
    let start = c + Complex::new(
        epsilon * (1 - super_samples) as f64,
        epsilon * (1 - super_samples) as f64,
    );
    let mut total = 0;
    for i in 0..super_samples {
        let re = epsilon * (i as f64);
        for j in 0..super_samples {
            let im = epsilon * (j as f64);
            total += sample(start + Complex::new(re, im), max_iter);
        }
    }
    total
}

fn sample_area(
    centre: Complex,
    scale: f64,
    res: [usize; 2],
    super_samples: i32,
    max_iter: i32,
    data: &mut Array2<i32>,
) {
    let aspect_ratio = res[0] as f64 / res[1] as f64;
    let start = centre + Complex::new(scale * -0.5, scale / aspect_ratio * -0.5);
    let delta = scale / (res[0] - 1).max(1) as f64;
    let epsilon = delta / (2 * super_samples) as f64;

    for yi in 0..res[1] {
        let y = start.im + (delta * yi as f64);
        for xi in 0..res[0] {
            let x = start.re + (delta * xi as f64);
            let c = Complex::new(x, y);
            data[(xi, yi)] = multi_sample(c, max_iter, super_samples, epsilon);
        }
    }
}

fn data_to_col() {}

fn col_to_image() {}

pub fn image(centre: Complex, scale: f64, res: [usize; 2], super_samples: i32, max_iter: i32) {}
