pub mod complex;
pub mod cpu;
pub mod gpu;
pub mod util;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn mandelbrot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<complex::Complex>()?;
    m.add_function(wrap_pyfunction!(cpu::sample, m)?)?;
    m.add_function(wrap_pyfunction!(cpu::multi_sample, m)?)?;
    m.add_function(wrap_pyfunction!(cpu::render_image, m)?)?;
    m.add_function(wrap_pyfunction!(cpu::render_video, m)?)?;
    m.add_function(wrap_pyfunction!(gpu::render_image, m)?)?;
    Ok(())
}
