use ndarray::{Array2, Array3};
use palette::{Gradient, LinSrgb};
use pyo3::prelude::*;
use std::path::Path;

use crate::{
    complex::Complex,
    util::{cols_to_image, data_to_cols},
};

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
pub fn multi_sample(c: Complex, max_iter: i32, super_samples: i32, epsilon: f64) -> f64 {
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
    total as f64 / (super_samples * super_samples) as f64
}

/// Sample a rectangular region of the complex plane.
/// The scale input defines the width of the real-axis sampled.
/// The aspect ratio is constrained to unity, so the resolution will determine the height of the imaginary-axis samples.
/// data is a two-dimensional array of scalar samples which can be used to store results.
/// We pass it in here to avoid having to re-allocate memory each time.
fn sample_area(
    centre: Complex,
    scale: f64,
    res: [usize; 2],
    super_samples: i32,
    max_iter: i32,
    data: &mut Array2<f64>,
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

/// Sample a rectangular region of the complex plane,
/// and then save the scalar samples as a coloured image.
#[pyfunction]
pub fn render_image(
    centre: Complex,
    scale: f64,
    res: [usize; 2],
    super_samples: i32,
    max_iter: i32,
    output_dir: String,
) {
    let cmap = Gradient::new(vec![
        LinSrgb::new(0.00, 0.05, 0.20),
        LinSrgb::new(0.70, 0.10, 0.20),
        LinSrgb::new(0.95, 0.90, 0.30),
    ]);

    let mut data = Array2::<f64>::zeros(res);
    let mut cols = Array3::<u8>::zeros((res[0], res[1], 3));

    sample_area(centre, scale, res, super_samples, max_iter, &mut data);
    data_to_cols(&data, max_iter, &cmap, &mut cols);
    cols_to_image(&cols)
        .save(Path::new(&output_dir).join(format!("img_{:04}.png", 0)))
        .expect("Failed to save image.");
}

/// Repeatedly render a ever increasing/decreasing region of the complex plane.
/// The scale at each iteration is multiplied by the given rate.
/// The max_iter is increased at each iteration.
#[pyfunction]
pub fn render_video(
    centre: Complex,
    mut scale: f64,
    rate: f64,
    res: [usize; 2],
    frames: usize,
    super_samples: i32,
    max_iter: i32,
    output_dir: String,
) {
    let cmap = Gradient::new(vec![
        LinSrgb::new(0.00, 0.05, 0.20),
        LinSrgb::new(0.70, 0.10, 0.20),
        LinSrgb::new(0.95, 0.90, 0.30),
    ]);

    let mut data = Array2::<f64>::zeros(res);
    let mut cols = Array3::<u8>::zeros((res[0], res[1], 3));

    for n in 0..frames {
        println!(
            "Frame {} of {} \t ({}x)",
            n + 1,
            frames,
            (1.0 / scale).log10() as i32
        );

        sample_area(
            centre,
            scale,
            res,
            super_samples,
            max_iter + n as i32,
            &mut data,
        );
        data_to_cols(&data, max_iter + n as i32, &cmap, &mut cols);
        cols_to_image(&cols)
            .save(Path::new(&output_dir).join(format!("img_{:04}.png", n)))
            .expect("Failed to save image.");

        scale *= rate;
    }
}
