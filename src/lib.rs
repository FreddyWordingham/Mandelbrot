pub mod complex;
pub mod mandelbrot;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn mandelbrot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<complex::Complex>()?;
    m.add_function(wrap_pyfunction!(mandelbrot::sample, m)?)?;
    m.add_function(wrap_pyfunction!(mandelbrot::multi_sample, m)?)?;
    m.add_function(wrap_pyfunction!(mandelbrot::render_image, m)?)?;
    Ok(())
}
