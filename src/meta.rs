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
    next: String,
    #[pyo3(get)]
    cursors: Cursor
}
