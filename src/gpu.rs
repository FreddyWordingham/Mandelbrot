use ndarray::{arr1, s, Array3};
use ocl::ProQue;
use palette::{Gradient, LinSrgb, Pixel};
use pyo3::prelude::*;
use std::path::Path;

use crate::{complex::Complex, util::cols_to_image};

#[pyfunction]
#[pyo3(name = "gpu_render_image")]
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

    let src = format!(
        "
        __kernel void mandelbrot(__global uint* buffer, uint width, uint height, uint max_iterations) {{
            int re = get_global_id(0);
            int im = get_global_id(1);

            float x0 = ((float)re / width) * 3.5 - 2.5;
            float y0 = ((float)im / height) * 2.0 - 1.0;
            float x = 0.0;
            float y = 0.0;
            float x2 = 0.0;
            float y2 = 0.0;
            uint iteration = 0;

            while (((x2 + y2) <= 4.0) && (iteration < max_iterations)) {{
                y = (x + x) * y + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                iteration = iteration + 1;
            }}

            buffer[(width * im) + re] = iteration;
        }}
    "
    );

    let pro_que = ProQue::builder()
        .src(src)
        .dims((res[0], res[1]))
        .build()
        .unwrap();
    let buffer = pro_que.create_buffer::<u32>().unwrap();
    let kernel = pro_que
        .kernel_builder("mandelbrot")
        .arg(&buffer)
        .arg(res[0])
        .arg(res[1])
        .arg(max_iter as u32)
        .build()
        .unwrap();

    unsafe { kernel.enq().unwrap() };

    let mut cols = Array3::<u8>::zeros((res[0], res[1], 3));
    let max = max_iter as f32;
    let mut vec = vec![0u32; buffer.len()];
    buffer.read(&mut vec).enq().unwrap();
    for (idx, iteration) in vec.iter().enumerate() {
        let xi = idx as usize % res[0] as usize;
        let yi = idx as usize / res[0] as usize;

        let col = cmap.get(*iteration as f32 / max);
        let u8s: [u8; 3] = col.into_format().into_raw();
        cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
    }

    cols_to_image(&cols)
        .save(Path::new(&output_dir).join(format!("img_{:04}.png", 0)))
        .expect("Failed to save image.");
}
