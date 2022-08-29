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

    let aspect_ratio = res[0] as f64 / res[1] as f64;
    let start = centre + Complex::new(scale * -0.5, scale / aspect_ratio * -0.5);
    let delta = scale / (res[0] - 1).max(1) as f64;
    let epsilon = delta / (2 * super_samples) as f64;

    let src = format!(
        "
        __kernel void mandelbrot(__global uint* buffer, float start_re, float start_im, uint width, uint super_samples, float delta, float epsilon, uint max_iter) {{
            int re = get_global_id(0);
            int im = get_global_id(1);

            float start_x = ((float)re * delta) + start_re;
            float start_y = ((float)im * delta) + start_im;

            float total = 0.0;
            for (int i = 0; i < super_samples; i++) {{
                float re_offset = epsilon * i;
                for (int j = 0; j < super_samples; j++) {{
                    float im_offset = epsilon * j;

                    float x0 = start_x + re_offset;
                    float y0 = start_y + im_offset;
                    float x = 0.0;
                    float y = 0.0;
                    float x2 = 0.0;
                    float y2 = 0.0;
                    uint iteration = 0;

                    while (((x2 + y2) <= 4.0) && (iteration < max_iter)) {{
                        y = (x + x) * y + y0;
                        x = x2 - y2 + x0;
                        x2 = x * x;
                        y2 = y * y;
                        iteration = iteration + 1;
                    }}

                    total += (float)iteration;
                }}
            }}

            buffer[(width * im) + re] = (uint)(total / (super_samples * super_samples));
        }}
    "
    );

    let pro_que = ProQue::builder()
        .src(src)
        .dims((res[0], res[1]))
        .build()
        .unwrap();
    let gpu_buffer = pro_que.create_buffer::<u32>().unwrap();
    let kernel = pro_que
        .kernel_builder("mandelbrot")
        .arg(&gpu_buffer)
        .arg(start.re as f32)
        .arg(start.im as f32)
        .arg(res[0])
        .arg(super_samples as u32)
        .arg(delta as f32)
        .arg(epsilon as f32)
        .arg(max_iter as u32)
        .build()
        .unwrap();
    unsafe { kernel.enq().unwrap() };

    let mut cols = Array3::<u8>::zeros((res[0], res[1], 3));
    let mut cpu_buffer = vec![0u32; gpu_buffer.len()];
    gpu_buffer.read(&mut cpu_buffer).enq().unwrap();
    for (idx, iteration) in cpu_buffer.iter().enumerate() {
        let xi = idx as usize % res[0] as usize;
        let yi = idx as usize / res[0] as usize;

        let col = cmap.get(*iteration as f32 / max_iter as f32);
        let u8s: [u8; 3] = col.into_format().into_raw();
        cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
    }

    cols_to_image(&cols)
        .save(Path::new(&output_dir).join(format!("img_{:04}.png", 0)))
        .expect("Failed to save image.");
}

#[pyfunction]
#[pyo3(name = "gpu_render_video")]
pub fn render_video(
    centre: Complex,
    mut scale: f64,
    rate: f64,
    res: [usize; 2],
    frames: usize,
    super_samples: i32,
    mut max_iter: i32,
    output_dir: String,
) {
    let cmap = Gradient::new(vec![
        LinSrgb::new(0.00, 0.05, 0.20),
        LinSrgb::new(0.70, 0.10, 0.20),
        LinSrgb::new(0.95, 0.90, 0.30),
    ]);

    let aspect_ratio = res[0] as f64 / res[1] as f64;

    let src = format!(
        "
        __kernel void mandelbrot(__global uint* buffer, float start_re, float start_im, uint width, uint super_samples, float delta, float epsilon, uint max_iter) {{
            int re = get_global_id(0);
            int im = get_global_id(1);

            float start_x = ((float)re * delta) + start_re;
            float start_y = ((float)im * delta) + start_im;

            float total = 0.0;
            for (int i = 0; i < super_samples; i++) {{
                float re_offset = epsilon * i;
                for (int j = 0; j < super_samples; j++) {{
                    float im_offset = epsilon * j;

                    float x0 = start_x + re_offset;
                    float y0 = start_y + im_offset;
                    float x = 0.0;
                    float y = 0.0;
                    float x2 = 0.0;
                    float y2 = 0.0;
                    uint iteration = 0;

                    while (((x2 + y2) <= 4.0) && (iteration < max_iter)) {{
                        y = (x + x) * y + y0;
                        x = x2 - y2 + x0;
                        x2 = x * x;
                        y2 = y * y;
                        iteration = iteration + 1;
                    }}

                    total += (float)iteration;
                }}
            }}

            buffer[(width * im) + re] = (uint)(total / (super_samples * super_samples));
        }}
    "
    );
    let pro_que = ProQue::builder()
        .src(src)
        .dims((res[0], res[1]))
        .build()
        .unwrap();
    let gpu_buffer = pro_que.create_buffer::<u32>().unwrap();

    let mut cols = Array3::<u8>::zeros((res[0], res[1], 3));
    let mut cpu_buffer = vec![0u32; gpu_buffer.len()];
    for n in 0..frames {
        println!(
            "Frame {} of {} \t ({}x)",
            n + 1,
            frames,
            (1.0 / scale).log10() as i32
        );

        let start = centre + Complex::new(scale * -0.5, scale / aspect_ratio * -0.5);
        let delta = scale / (res[0] - 1).max(1) as f64;
        let epsilon = delta / (2 * super_samples) as f64;

        let kernel = pro_que
            .kernel_builder("mandelbrot")
            .arg(&gpu_buffer)
            .arg(start.re as f32)
            .arg(start.im as f32)
            .arg(res[0])
            .arg(super_samples as u32)
            .arg(delta as f32)
            .arg(epsilon as f32)
            .arg(max_iter as u32)
            .build()
            .unwrap();
        unsafe { kernel.enq().unwrap() };

        gpu_buffer.read(&mut cpu_buffer).enq().unwrap();
        for (idx, iteration) in cpu_buffer.iter().enumerate() {
            let xi = idx as usize % res[0] as usize;
            let yi = idx as usize / res[0] as usize;

            let col = cmap.get(*iteration as f32 / max_iter as f32);
            let u8s: [u8; 3] = col.into_format().into_raw();
            cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
        }

        cols_to_image(&cols)
            .save(Path::new(&output_dir).join(format!("img_{:04}.png", n)))
            .expect("Failed to save image.");

        scale *= rate;
        max_iter += 1;
    }
}
