use pyo3::prelude::*;
use std::env;
use dotenv::dotenv;
use pyo3::exceptions::PyValueError;

/// Adds two numbers.
#[pyfunction]
fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[pyfunction]
fn get_meta_client_id() -> PyResult<String> {
    dotenv().ok();

    match env::var("META_CLIENT_ID") {
        Ok(client_id) => Ok(client_id),
        Err(_) => Err(PyValueError::new_err("META_CLIENT_ID not set in environment")),
    }
}

#[pyfunction]
fn get_meta_client_secret() -> PyResult<String> {
    dotenv().ok();

    match env::var("META_CLIENT_SECRET") {
        Ok(client_secret) => Ok(client_secret),
        Err(_) => Err(PyValueError::new_err("META_CLIENT_SECRET not set in environment")),
    }
}

/// Python module definition
#[pymodule]
fn meta_stats(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(get_meta_client_id, m)?)?;
    m.add_function(wrap_pyfunction!(get_meta_client_secret, m)?)?;
    Ok(())
}
