use pyo3::prelude::*;

/// A simple Rust function exposed to Python
#[pyfunction]
fn hash_message(message: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(message);
    format!("{:x}", hasher.finalize())
}

/// The module entry point
#[pymodule]
fn rust_backend(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash_message, m)?)?;
    Ok(())
}


// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
