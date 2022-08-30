use pyo3::prelude::*;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

/// Complex number representation.
#[pyclass]
#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[pymethods]
impl Complex {
    /// Construct a complex number with a normal of zero.
    #[staticmethod]
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    /// Construct a new complex number from the given real and imaginary components.
    #[new]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Create a string representation of the complex number.
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    /// Create a string representation of the complex number.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

impl Complex {
    /// Compute the normal squared of the complex number.
    pub fn norm_squared(self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }

    /// Compute the normal of the complex number.
    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    /// Add two complex numbers together.
    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    /// Multiply this complex number by a given scalar.
    fn mul(self, other: f64) -> Self {
        Self {
            re: self.re * other,
            im: self.re * other,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    /// Multiply two complex numbers together.
    fn mul(self, other: Self) -> Self {
        Self {
            re: (self.re * other.re) - (self.im * other.im),
            im: (self.re * other.im) + (self.im * other.re),
        }
    }
}

impl Display for Complex {
    /// Write a string representation of the complex number to the given formatter.
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.re, self.im)
    }
}
