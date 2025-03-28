use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use pyo3::exceptions::PyValueError;
use reqwest::blocking::Client;
use rand::seq::SliceRandom;
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


const USER_AGENTS: [&str; 11] = [
    "Instagram 241.1.0.18.114 Android (31/12; 420dpi; 1080x2148; samsung; SM-G998B; o1s; exynos2100; en_US; 378436363)",
    "Instagram 244.0.0.17.110 Android (30/11; 480dpi; 1080x2400; xiaomi; M2103K19PG; dandelion; qcom; en_US; 383877306)",
    "Instagram 245.0.0.18.108 Android (28/9; 320dpi; 720x1382; OnePlus; ONEPLUS A3003; OnePlus3; qcom; en_US; 385416158)",
    "Instagram 239.0.0.14.111 Android (29/10; 480dpi; 1080x2310; Xiaomi; Redmi Note 8 Pro; begonia; mt6785; en_US; 373310557)",
    "Instagram 243.1.0.14.111 (iPhone13,4; iOS 16_5_1; en_US; en-US; scale=3.00; 1284x2778; 382468103)",
    "Instagram 244.0.0.15.110 (iPhone14,3; iOS 17_1; en_US; en-US; scale=3.00; 1170x2532; 384888781)",
    "Instagram 246.0.0.17.107 (iPhone12,1; iOS 15_7_9; en_US; en-US; scale=2.00; 828x1792; 387311294)",
    "Instagram 242.0.0.12.109 (iPad13,1; iOS 16_6; en_US; en-US; scale=2.00; 1620x2160; 381295754)",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
];

const APP_IDS: [&str; 5] = [
    "936619743392459",
    "124024574287414",
    "389801252",
    "503323726800545",
    "446889149701729",
];


#[pyfunction]
pub fn get_instagram_page_info(username: Option<String>) -> PyResult<Py<InstagramPageInfoResult>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| PyValueError::new_err(format!("Failed to create HTTP client for instagram page info: {}", e)))?;
    
    let user_agent = USER_AGENTS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&USER_AGENTS[0]);

    let app_id = APP_IDS.choose(&mut rand::thread_rng()).unwrap_or(&"936619743392459");


    match username {
        Some(username) => {
            let url = format!("https://i.instagram.com/api/v1/users/web_profile_info/?username={}", username);
            let res = client.get(&url)
                .header("Accept-Language", "en-US")
                .header("User-Agent", *user_agent)
                .header("X-IG-App-ID", *app_id)
                .header("X-IG-Capabilities", "3brTvw==")
                .header("X-IG-Connection-Type", "WIFI")
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