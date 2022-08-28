use image::RgbImage;
use ndarray::{arr1, s, Array2, Array3};
use palette::{Gradient, LinSrgb, Pixel};

pub fn data_to_cols(
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

pub fn cols_to_image(arr: &Array3<u8>) -> RgbImage {
    let (width, height, _) = arr.dim();
    RgbImage::from_vec(
        width as u32,
        height as u32,
        arr.as_slice().unwrap().to_vec(),
    )
    .expect("container should have the right size for the image dimensions")
}
