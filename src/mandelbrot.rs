use crate::complex::Complex;
use image::RgbImage;
use ndarray::{arr1, s, Array2, Array3};
use palette::{Gradient, LinSrgb, Pixel};
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

fn data_to_cols(
    data: &Array2<f64>,
    max_iter: i32,
    cmap: &Gradient<LinSrgb>,
    cols: &mut Array3<u8>,
) {
    let max = max_iter as f32;

    let (width, height) = data.dim();
    for yi in 0..height {
        for xi in 0..width {
            let col = cmap.get(data[(xi, yi)] as f32 / max);
            let u8s: [u8; 3] = col.into_format().into_raw();
            cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
        }
    }
}

fn cols_to_image(arr: Array3<u8>) -> RgbImage {
    let (width, height, _) = arr.dim();
    RgbImage::from_raw(width as u32, height as u32, arr.into_raw_vec())
        .expect("container should have the right size for the image dimensions")
}

#[pyfunction]
pub fn render_image(
    centre: Complex,
    scale: f64,
    res: [usize; 2],
    super_samples: i32,
    max_iter: i32,
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
    cols_to_image(cols)
        .save(format!("output/img_{:04}.png", 0))
        .expect("Failed to save image.");
}
