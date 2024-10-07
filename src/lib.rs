use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use blake2::{Blake2b, Blake2s, Blake2bVar, digest::{ VariableOutput}};
use digest::{consts::U32, Digest};
use base58::{ToBase58};
/// Function to hash data with Blake2b
#[pyfunction]
fn blake2b_hash(data: &[u8]) -> PyResult<String> {
    let mut hasher = Blake2b::<U32>::new();
    hasher.update(data);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Function to hash data with Blake2bVar
#[pyfunction]
fn blake2bvar_hash(data: &[u8], output_size: usize, version: String) -> PyResult<String> {
    use blake2::digest::Update;
    let mut hasher = Blake2bVar::new(output_size).unwrap();
    hasher.update(data);
    let mut buf = [0u8; 20];
    hasher.finalize_variable(&mut buf).unwrap();
    // let mode = MiningMode::to_str(config.mode());
    let unique_string = format!("{}", buf.to_base58());
    Ok(unique_string)

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
    m.add_function(wrap_pyfunction!(blake2bvar_hash, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake2b_hash() {
        let data = b"hello world";
        let expected_hash = "256c335dd3f6db5c5ad792250c35fa117ad0af98b99e409596fc29c88b627240"; // Expected hex hash
        let hash = blake2b_hash(data).unwrap();
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_blake2s_hash() {
        let data = b"hello world";
        let expected_hash = "30d8f5d02b111a64f05fa818e9e088c5102ac18f03bcf244eefebddb9562373b"; // Expected hex hash
        let hash = blake2s_hash(data).unwrap();
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_blake2bvar_hash() {
        let data = b"hello world";
        let output_size = 20;
        let version = "1.0.0".to_string();
        let hash = blake2bvar_hash(data, output_size, version.clone()).unwrap();

        // Example format of the output string: "v2,<base58_encoded_hash>,<version>"
        assert!(hash.starts_with("v2,")); // Verify the prefix
        assert!(hash.contains(&version)); // Ensure the version is part of the output
    }
}