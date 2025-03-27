use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use pyo3::exceptions::PyValueError;
use reqwest::blocking::Client;
use crate::meta;

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FollowedBy {
    #[pyo3(get)]
    #[serde(default)]
    count: i32,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UserInfo {
    #[pyo3(get)]
    #[serde(default)]
    eimu_id: String,

    #[pyo3(get)]
    #[serde(default)]
    biography: String,

    #[pyo3(get)]
    #[serde(default)]
    edge_followed_by: FollowedBy,

    #[pyo3(get)]
    #[serde(default)]
    full_name: String,

    #[pyo3(get)]
    #[serde(default)]
    highlight_reel_count: i32,

    #[pyo3(get)]
    #[serde(default)]
    category_name: Option<String>,

}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InstagramPageData {
    #[pyo3(get)]
    #[serde(default)]
    user: UserInfo,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct InstagramPageInfo {
    #[pyo3(get)]
    #[serde(default)]
    data: InstagramPageData,

    #[pyo3(get)]
    #[serde(default)]
    status: String,
}

#[pyclass]
#[derive(Clone)]
pub struct InstagramPageInfoResult {
    is_success: bool,
    page_info: Option<InstagramPageInfo>,
    error: Option<meta::MetaInstagramError>,
}


#[pymethods]
impl InstagramPageInfoResult {
    #[staticmethod]
    fn success(info: InstagramPageInfo) -> Self {
        InstagramPageInfoResult {
            is_success: true,
            page_info: Some(info),
            error: None,
        }
    }

    #[staticmethod]
    fn error(err: meta::MetaInstagramError) -> Self {
        InstagramPageInfoResult {
            is_success: false,
            page_info: None,
            error: Some(err),
        }
    }

    #[getter]
    fn is_success(&self) -> bool {
        self.is_success
    }
    
    #[getter]
    fn is_error(&self) -> bool {
        !self.is_success
    }
    
    #[getter]
    fn page_info(&self) -> Option<Py<InstagramPageInfo>> {
        match &self.page_info {
            Some(info) => Python::with_gil(|py| Some(Py::new(py, info.clone()).unwrap())),
            None => None,
        }
    }
    
    #[getter]
    fn error_info(&self) -> Option<Py<meta::MetaInstagramError>> {
        match &self.error {
            Some(err) => Python::with_gil(|py| Some(Py::new(py, err.clone()).unwrap())),
            None => None,
        }
    }
    
    fn __repr__(&self) -> PyResult<String> {
        if self.is_success {
            Ok("InstagramPageInfoResult(Success)".to_string())
        } else {
            Ok("InstagramPageInfoResult(Error)".to_string())
        }
    }
}


const USER_AGENT: &str = "Instagram 241.1.0.18.114 Android (31/12; 420dpi; 1080x2148; samsung; SM-G998B; o1s; exynos2100; en_US; 378436363)";


#[pyfunction]
pub fn get_instagram_page_info(username: Option<String>) -> PyResult<Py<InstagramPageInfoResult>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| PyValueError::new_err(format!("Failed to create HTTP client for instagram page info: {}", e)))?;
    

    match username {
        Some(username) => {
            let url = format!("https://i.instagram.com/api/v1/users/web_profile_info/?username={}", username);
            let res = client.get(&url)
                .header("User-Agent", USER_AGENT)
                .send()
                .map_err(|e| PyValueError::new_err(format!("Request failed in instagram page info: {}", e)))?;

            let status = res.status();
            let raw_text = res.text()
                .map_err(|e| PyValueError::new_err(format!("Failed to get response text: {}", e)))?;
 
            if raw_text.contains("\"status\":\"ok\"") {
                match serde_json::from_str::<InstagramPageInfo>(&raw_text) {
                    Ok(page_info) => {
                        let result = InstagramPageInfoResult::success(page_info);
                        return Python::with_gil(|py| {
                            Py::new(py, result).map_err(|e| PyErr::from(e))
                        });
                    },
                    Err(e) => {
                        return Err(PyValueError::new_err(
                            format!("Response looks like success but failed to parse to InstagramPageInfo: {}", e)
                        ));
                    }
                }
            }
            
            match serde_json::from_str::<meta::MetaInstagramError>(&raw_text) {
                Ok(error) => {
                    let result = InstagramPageInfoResult::error(error);
                    return Python::with_gil(|py| {
                        Py::new(py, result).map_err(|e| PyErr::from(e))
                    });
                },
                Err(e) => {
                    return Err(PyValueError::new_err(format!(
                        "Failed to parse response to InstagramPageInfo as either success or error. Status: {}, Parse error: {}, Response preview: {:.200}...", 
                        status, e, raw_text
                    )));
                }
            }
        },
        None => Err(PyValueError::new_err("username must be set to the username of the page to get info for"))?,
    }


}