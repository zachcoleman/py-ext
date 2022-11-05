use pyo3::prelude::*;

mod hello;

#[pymodule]
fn rust_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello::hello_world, m)?)?;
    Ok(())
}