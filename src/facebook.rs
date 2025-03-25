use crate::auth;
use crate::meta;
use pyo3::prelude::*;
//use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::PyValueError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BasicPageInfo {
    #[pyo3(get)]
    #[serde(default)]
    id: String,
    #[pyo3(get)]
    #[serde(default)]
    name: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PageCategory {
    #[pyo3(get)]
    #[serde(default)]
    id: String,
    #[pyo3(get)]
    #[serde(default)]
    name: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PageEngagement {
    #[pyo3(get)]
    #[serde(default)]
    category: String,
    #[pyo3(get)]
    #[serde(default)]
    category_list: Vec<PageCategory>,
    #[pyo3(get)]
    #[serde(default)]
    followers_count: i32,
    #[pyo3(get)]
    #[serde(default)]
    fan_count: i32,
    #[pyo3(get)]
    #[serde(default)]
    overall_star_rating: f32,
    #[pyo3(get)]
    #[serde(default)]
    rating_count: i32,
    #[pyo3(get)]
    #[serde(default)]
    talking_about_count: i32,
    #[pyo3(get)]
    #[serde(default)]
    id : String,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CommentInfo {
    #[pyo3(get)]
    message: Option<String>,
    #[pyo3(get)]
    created_time: Option<String>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LikeInfo {
    #[pyo3(get)]
    id: Option<String>,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LikeSummary {
    #[pyo3(get)]
    #[serde(default)]
    total_count: i32,
    #[pyo3(get)]
    #[serde(default)]
    can_like: bool,
    #[pyo3(get)]
    #[serde(default)]
    has_liked: bool,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CommentSummary {
    #[pyo3(get)]
    #[serde(default)]
    order: String,
    #[pyo3(get)]
    #[serde(default)]
    total_count: i32,
    #[pyo3(get)]
    #[serde(default)]
    can_comment: bool,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BasicCommentsInfo {
    #[pyo3(get)]
    #[serde(default)]
    data: Vec<CommentInfo>,
    #[pyo3(get)]
    paging: Option<meta::Paging>,
    #[pyo3(get)]
    #[serde(default)]
    summary: CommentSummary,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct BasicLikesInfo {
    #[pyo3(get)]
    #[serde(default)]
    data: Vec<LikeInfo>,
    #[pyo3(get)]
    paging: Option<meta::Paging>,
    #[pyo3(get)]
    #[serde(default)]
    summary: LikeSummary,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PostInfo {
    #[pyo3(get)]
    #[serde(default)]
    id: String,
    #[pyo3(get)]
    message: Option<String>,
    #[pyo3(get)]
    #[serde(default)]
    created_time: String,
    #[pyo3(get)]
    story: Option<String>,
    #[pyo3(get)]
    likes: Option<BasicLikesInfo>,
    #[pyo3(get)]
    comments: Option<BasicCommentsInfo>,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct BasicPostsInfo {
    #[pyo3(get)]
    data: Vec<PostInfo>,
    #[pyo3(get)]
    paging: meta::Paging,

}

#[pyclass]
#[derive(Clone)]
pub struct BasicPageInfoResult {
    is_success: bool,
    page_info: Option<BasicPageInfo>,
    error: Option<meta::MetaError>,
}

#[pymethods]
impl BasicPageInfoResult {
    #[staticmethod]
    fn success(info: BasicPageInfo) -> Self {
        BasicPageInfoResult {
            is_success: true,
            page_info: Some(info),
            error: None,
        }
    }

    #[staticmethod]
    fn error(err: meta::MetaError) -> Self {
        BasicPageInfoResult {
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
    fn page_info(&self) -> Option<Py<BasicPageInfo>> {
        match &self.page_info {
            Some(info) => Python::with_gil(|py| Some(Py::new(py, info.clone()).unwrap())),
            None => None,
        }
    }

    #[getter]
    fn get_error(&self) -> Option<Py<meta::MetaError>> {
        match &self.error {
            Some(err) => Python::with_gil(|py| Some(Py::new(py, err.clone()).unwrap())),
            None => None,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        if self.is_success {
            Ok("BasicPageInfoResult(Success)".to_string())
        } else {
            Ok("BasicPageInfoResult(Error)".to_string())
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PageEngagementResult {
    is_success: bool,
    page_engagement: Option<PageEngagement>,
    error: Option<meta::MetaError>,
}

#[pymethods]
impl PageEngagementResult {

    #[staticmethod]
    fn success(info: PageEngagement) -> Self {
        PageEngagementResult {
            is_success: true,
            page_engagement: Some(info),
            error: None,
        }
    }

    #[staticmethod]
    fn error(err: meta::MetaError) -> Self {
        PageEngagementResult {
            is_success: false,
            page_engagement: None,
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
    fn page_engagement(&self) -> Option<Py<PageEngagement>> {
        match &self.page_engagement {
            Some(info) => Python::with_gil(|py| Some(Py::new(py, info.clone()).unwrap())),
            None => None,
        }
    }

    #[getter]
    fn get_error(&self) -> Option<Py<meta::MetaError>> {
        match &self.error {
            Some(err) => Python::with_gil(|py| Some(Py::new(py, err.clone()).unwrap())),
            None => None,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        if self.is_success {
            Ok("PageEngagementResult(Success)".to_string())
        } else {
            Ok("PageEngagementResult(Error)".to_string())
        }
    }
}


#[pyclass]
#[derive(Clone)]
pub struct BasicPostsInfoResult {
    is_success: bool,
    posts_info: Option<BasicPostsInfo>,
    error: Option<meta::MetaError>,
}

#[pymethods]
impl BasicPostsInfoResult {
    #[staticmethod]
    fn success(info: BasicPostsInfo) -> Self {
        BasicPostsInfoResult {
            is_success: true,
            posts_info: Some(info),
            error: None,
        }
    }
 
    #[staticmethod]
    fn error(err: meta::MetaError) -> Self {
        BasicPostsInfoResult {
            is_success: false,
            posts_info: None,
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
    fn posts_info(&self) -> Option<Py<BasicPostsInfo>> {
        match &self.posts_info {
            Some(info) => Python::with_gil(|py| Some(Py::new(py, info.clone()).unwrap())),
            None => None,
        }
    }

    #[getter]
    fn get_error(&self) -> Option<Py<meta::MetaError>> {
        match &self.error {
            Some(err) => Python::with_gil(|py| Some(Py::new(py, err.clone()).unwrap())),
            None => None,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        if self.is_success {
            Ok("BasicPostsInfoResult(Success)".to_string())
        } else {
            Ok("BasicPostsInfoResult(Error)".to_string())
        }
    }
}


#[pyfunction]
pub fn get_facebook_page_info(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<BasicPageInfoResult>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| PyValueError::new_err(format!("Failed to create HTTP client for facebook page info: {}", e)))?;



    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };


    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}?access_token={}", meta_version, page_id, access_token);
            let res = client
                .get(&url)
                .send()
                .map_err(|e| PyValueError::new_err(format!("Request failed for BasicPageInfo: {}", e)))?;
            
            let status = res.status();
            let raw_text = res
                .text()
                .map_err(|e| PyValueError::new_err(format!("Failed to get response text for BasicPageInfo: {}", e)))?;
            
            let error_result: Result<meta::MetaError, _> = serde_json::from_str(&raw_text);
            
            if let Ok(error) = error_result {
                let result = BasicPageInfoResult::error(error);
                return Python::with_gil(|py| {
                    Py::new(py, result).map_err(|e| PyErr::from(e))
                });
            }
            
            match serde_json::from_str::<BasicPageInfo>(&raw_text) {
                Ok(page_info) => {
                    let result = BasicPageInfoResult::success(page_info);
                    Python::with_gil(|py| {
                        Py::new(py, result).map_err(|e| PyErr::from(e))
                    })
                },
                Err(e) => {
                    eprintln!("Failed to parse Facebook response: {}", e);
                    eprintln!("Raw response: {}", raw_text);
                    
                    Err(PyValueError::new_err(format!(
                        "Failed to parse response as either a Facebook page or error. Status: {}, Parse error: {}, Response preview: {:.200}...",
                        status, e, raw_text
                    )))
                }
            }
        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }

}

#[pyfunction]
pub fn get_facebook_page_followers(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<PageEngagementResult>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| PyValueError::new_err(format!("Failed to create HTTP client for facebook page followers: {}", e)))?;


    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };

    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}?fields=category,category_list,followers_count,fan_count,new_like_count,overall_star_rating,rating_count,talking_about_count&access_token={}", meta_version, page_id, access_token);
            let res = client
                .get(&url)
                .send()
                .map_err(|e| PyValueError::new_err(format!("Request failed for PageEngagement: {}", e)))?;
            
            let status = res.status();
            let raw_text = res
                .text()
                .map_err(|e| PyValueError::new_err(format!("Failed to get response text for PageEngagement: {}", e)))?;
            
            let error_result: Result<meta::MetaError, _> = serde_json::from_str(&raw_text);
            
            if let Ok(error) = error_result {
                let result = PageEngagementResult::error(error);
                return Python::with_gil(|py| {
                    Py::new(py, result).map_err(|e| PyErr::from(e))
                });
            }
            
            match serde_json::from_str::<PageEngagement>(&raw_text) {
                Ok(page_info) => {
                    let result = PageEngagementResult::success(page_info);
                    Python::with_gil(|py| {
                        Py::new(py, result).map_err(|e| PyErr::from(e))
                    })
                },
                Err(e) => {
                    eprintln!("Failed to parse Facebook response: {}", e);
                    eprintln!("Raw response: {}", raw_text);
                    
                    Err(PyValueError::new_err(format!(
                        "Failed to parse response as either a Facebook page or error. Status: {}, Parse error: {}, Response preview: {:.200}...",
                        status, e, raw_text
                    )))
                }
            }

        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }

}

#[pyfunction]
pub fn get_facebook_page_posts(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<BasicPostsInfoResult>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| PyValueError::new_err(format!("Failed to create HTTP client for facebook page posts: {}", e)))?;

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };

    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}/posts?&access_token={}", meta_version, page_id, access_token);
            let res = client
                .get(&url)
                .send()
                .map_err(|e| PyValueError::new_err(format!("Request failed for BasicPostsInfo: {}", e)))?;
            
            let status = res.status();
            let raw_text = res
                .text()
                .map_err(|e| PyValueError::new_err(format!("Failed to get response text for BasicPostsInfo: {}", e)))?;
            
            let error_result: Result<meta::MetaError, _> = serde_json::from_str(&raw_text);

            if let Ok(error) = error_result {
                let result = BasicPostsInfoResult::error(error);
                return Python::with_gil(|py| {
                    Py::new(py, result).map_err(|e| PyErr::from(e))
                });
            }
            
            match serde_json::from_str::<BasicPostsInfo>(&raw_text) {
                Ok(page_info) => {
                    let result = BasicPostsInfoResult::success(page_info);
                    Python::with_gil(|py| {
                        Py::new(py, result).map_err(|e| PyErr::from(e))
                    })
                },
                Err(e) => {
                    eprintln!("Failed to parse Facebook response: {}", e);
                    eprintln!("Raw response: {}", raw_text);
                    
                    Err(PyValueError::new_err(format!(
                        "Failed to parse response as either a Facebook posts or error. Status: {}, Parse error: {}, Response preview: {:.200}...",
                        status, e, raw_text
                    )))
                }
            }


        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }

}

#[pyfunction]
pub fn get_facebook_page_posts_with_summary(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<BasicPostsInfoResult>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| PyValueError::new_err(format!("Failed to create HTTP client for facebook page posts: {}", e)))?;

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };

    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}/posts?fields=id,message,created_time,likes.summary(true),comments.summary(true)&access_token={}", meta_version, page_id, access_token);

            let res = client
                .get(&url)
                .send()
                .map_err(|e| PyValueError::new_err(format!("Request failed for BasicPostsInfo: {}", e)))?;
            
            let status = res.status();
            let raw_text = res
                .text()
                .map_err(|e| PyValueError::new_err(format!("Failed to get response text for BasicPostsInfo: {}", e)))?;
            
            let error_result: Result<meta::MetaError, _> = serde_json::from_str(&raw_text);

            
            if let Ok(error) = error_result {
                let result = BasicPostsInfoResult::error(error);
                return Python::with_gil(|py| {
                    Py::new(py, result).map_err(|e| PyErr::from(e))
                });
            }
            
            match serde_json::from_str::<BasicPostsInfo>(&raw_text) {
                Ok(page_info) => {
                    let result = BasicPostsInfoResult::success(page_info);
                    Python::with_gil(|py| {
                        Py::new(py, result).map_err(|e| PyErr::from(e))
                    })
                },
                Err(e) => {
                    eprintln!("Failed to parse Facebook response: {}", e);
                    eprintln!("Raw response: {}", raw_text);
                    
                    Err(PyValueError::new_err(format!(
                        "Failed to parse response as either a Facebook posts or error. Status: {}, Parse error: {}, Response preview: {:.200}...",
                        status, e, raw_text
                    )))
                }
            }

        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }
}

#[pyfunction]
pub fn get_facebook_next_results(next: String) -> PyResult<String> {

    let client = Client::new();

    let res = client.get(&next).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
    let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;

    Ok(raw_text)

}

#[pyfunction]
pub fn parse_next_results_to_basic_posts_info(raw_text: String) -> PyResult<Py<BasicPostsInfoResult>> {
    let error_result: Result<meta::MetaError, _> = serde_json::from_str(&raw_text);

            
    if let Ok(error) = error_result {
        let result = BasicPostsInfoResult::error(error);
        return Python::with_gil(|py| {
            Py::new(py, result).map_err(|e| PyErr::from(e))
        });
    }
    
    match serde_json::from_str::<BasicPostsInfo>(&raw_text) {
        Ok(page_info) => {
            let result = BasicPostsInfoResult::success(page_info);
            Python::with_gil(|py| {
                Py::new(py, result).map_err(|e| PyErr::from(e))
            })
        },
        Err(e) => {
            eprintln!("Failed to parse Facebook response: {}", e);
            eprintln!("Raw response: {}", raw_text);
            
            Err(PyValueError::new_err(format!(
                "Failed to parse response as either a Facebook posts or error. Parse error: {}, Response preview: {:.200}...",
                e, raw_text
            )))
        }
    }
}

#[pyfunction]
pub fn get_facebook_post_interactions(access_token: Option<String>, post_id: Option<String>, meta_version: Option<String>) -> PyResult<String> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::new();

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };

    match post_id {
        Some(post_id) => {
            let url = format!("https://graph.facebook.com/{}/{}/reactions?access_token={}", meta_version, post_id, access_token);
            let res = client.get(&url).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;

            Ok(raw_text)
        },
        None => Err(PyValueError::new_err("Post ID is required")),
    }
            
}