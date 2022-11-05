use cached::proc_macro::cached;
use pyo3::prelude::*;

#[pyfunction]
pub fn fib_no_cache(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_no_cache(n - 2) + fib_no_cache(n - 1)
    // _rust_fib_no_cache(n)
}

// fn _rust_fib_no_cache(n: u64) -> u64{
//     if n <= 1 {
//         return n;
//     }
//     fib_no_cache(n - 2) + fib_no_cache(n - 1)
// }

#[pyfunction]
#[cached]
pub fn fib_cache(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_cache(n - 2) + fib_cache(n - 1)
}
