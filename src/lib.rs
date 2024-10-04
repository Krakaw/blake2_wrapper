use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use blake2::{Blake2b, Blake2s};
use digest::{consts::U32, Digest};

/// Function to hash data with Blake2b
#[pyfunction]
fn blake2b_hash(data: &[u8]) -> PyResult<String> {
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Function to hash data with Blake2s
#[pyfunction]
fn blake2s_hash(data: &[u8]) -> PyResult<String> {
    let mut hasher = Blake2s::<U32>::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}


/// Module initialization
#[pymodule]
fn blake2_wrapper(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(blake2b_hash, m)?)?;
    m.add_function(wrap_pyfunction!(blake2s_hash, m)?)?;
    Ok(())
}
