use pyo3::prelude::*;

#[pyfunction]
pub fn hello_world() {
    println!("Hello World!");
}