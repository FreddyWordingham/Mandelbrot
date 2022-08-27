pub mod complex;
pub mod mandelbrot;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mandelbrot::sample, m)?)?;
    Ok(())
}
