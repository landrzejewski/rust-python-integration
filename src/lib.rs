// python3 -m maturin init
// python3 -m maturin build
// pip3 install localization-0.1.0-cp39-cp39-macosx_11_0_arm64.whl

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
