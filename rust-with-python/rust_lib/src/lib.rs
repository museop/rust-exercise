use pyo3::prelude::*;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[pyfunction]
fn sum_primes(limit: u64) -> PyResult<u64> {
    let mut sum = 0;
    for i in 2..=limit {
        if is_prime(i) {
            sum += i;
        }
    }
    Ok(sum)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_primes, m)?)?;
    Ok(())
}
