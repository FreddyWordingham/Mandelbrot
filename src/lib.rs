pub mod complex;

// use crate::complex::Complex;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn mandelbrot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sample, m)?)?;
    Ok(())
}

#[pyfunction]
fn sample(max_iter: i32) -> i32 {
    max_iter
}
