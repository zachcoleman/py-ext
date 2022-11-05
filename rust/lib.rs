use pyo3::prelude::*;

mod fib;
mod hello;

#[pymodule]
fn rust_ext(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello::hello_world, m)?)?;
    m.add_function(wrap_pyfunction!(fib::fib_no_cache, m)?)?;
    m.add_function(wrap_pyfunction!(fib::fib_cache, m)?)?;
    Ok(())
}
