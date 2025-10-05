use pyo3::prelude::*;

/// A stub module for pyquillmark.
/// This is a placeholder implementation to reserve the PyPI package name.
#[pymodule]
fn _core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
