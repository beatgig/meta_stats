use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct Cursor {
    #[pyo3(get)]
    before: String,
    #[pyo3(get)]
    after: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct Paging {
    #[pyo3(get)]
    next: Option<String>,
    #[pyo3(get)]
    cursors: Cursor
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct MetaErrorData {
    #[pyo3(get)]
    message: String,
    #[pyo3(get)]
    r#type: String,
    #[pyo3(get)]
    code: i32,
    #[pyo3(get)]
    fbtrace_id: String,
    #[pyo3(get)]
    is_transient: bool,
}
    

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct MetaError {
    #[pyo3(get)]
    error: MetaErrorData,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct MetaInstagramError {
    #[pyo3(get)]
    message: String,
    #[pyo3(get)]
    require_login: bool,
    #[pyo3(get)]
    igweb_rollout: bool,
    #[pyo3(get)]
    status: String,
}
