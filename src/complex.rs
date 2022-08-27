use pyo3::{prelude::*, PyObjectProtocol};
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
    #[staticmethod]
    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    #[new]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl Complex {
    pub fn norm_squared(self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }

    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            re: self.re * other,
            im: self.re * other,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: (self.re * other.re) - (self.im * other.im),
            im: (self.re * other.im) + (self.im * other.re),
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.re, self.im)
    }
}

#[pyproto]
impl PyObjectProtocol for Complex {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}
