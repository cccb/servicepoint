use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(name = "fnord")]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "rust_bindings")]
fn servicepoint_binding_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
