use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use pyo3::exceptions::PyValueError;
use reqwest::blocking::Client;

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct FollowedBy {
    #[pyo3(get)]
    count: i32,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct UserInfo {
    #[pyo3(get)]
    eimu_id: String,

    #[pyo3(get)]
    biography: String,

    #[pyo3(get)]
    edge_followed_by: FollowedBy,

    #[pyo3(get)]
    full_name: String,

    #[pyo3(get)]
    highlight_reel_count: i32,

    #[pyo3(get)]
    category_name: String,

}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct InstagramPageData {
    #[pyo3(get)]
    user: UserInfo,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct InstagramPageInfo {
    #[pyo3(get)]
    data: InstagramPageData,

    #[pyo3(get)]
    status: String,
}

#[pyfunction]
pub fn get_instagram_page_info(username: Option<String>) -> PyResult<Py<InstagramPageInfo>> {
    let client = Client::new();

    match username {
        Some(username) => {
            let url = format!("https://i.instagram.com/api/v1/users/web_profile_info/?username={}", username);
            let res = client.get(&url).header("User-Agent", "Instagram 76.0.0.15.395 Android (24/7.0; 640dpi; 1440x2560; samsung; SM-G930F; herolte; samsungexynos8890; en_US; 138226743)").send().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let page_info: InstagramPageInfo = serde_json::from_str(&raw_text)
                .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON response: {}", e)))?;
            
            Python::with_gil(|py| {
                Py::new(py, page_info).map_err(|e| PyErr::from(e))
            }) 
        },
        None => Err(PyValueError::new_err("profile_url must be set to the profile url of the page to get info for"))?,
    }


}