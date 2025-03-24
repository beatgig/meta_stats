use crate::auth;
use crate::meta;
use pyo3::prelude::*;
//use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::PyValueError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct BasicPageInfo {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    name: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct PageCategory {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    name: String,
}

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct PageEngagement {
    #[pyo3(get)]
    category: String,
    #[pyo3(get)]
    category_list: Vec<PageCategory>,
    #[pyo3(get)]
    followers_count: i32,
    #[pyo3(get)]
    fan_count: i32,
    #[pyo3(get)]
    overall_star_rating: f32,
    #[pyo3(get)]
    rating_count: i32,
    #[pyo3(get)]
    talking_about_count: i32,
    #[pyo3(get)]
    id : String,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct CommentInfo {
    #[pyo3(get)]
    message: Option<String>,
    #[pyo3(get)]
    created_time: Option<String>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct LikeInfo {
    #[pyo3(get)]
    id: Option<String>,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct LikeSummary {
    #[pyo3(get)]
    total_count: i32,
    #[pyo3(get)]
    can_like: bool,
    #[pyo3(get)]
    has_liked: bool,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct CommentSummary {
    #[pyo3(get)]
    order: String,
    #[pyo3(get)]
    total_count: i32,
    #[pyo3(get)]
    can_comment: bool,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct BasicCommentsInfo {
    #[pyo3(get)]
    data: Vec<CommentInfo>,
    #[pyo3(get)]
    paging: Option<meta::Paging>,
    #[pyo3(get)]
    summary: CommentSummary,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct BasicLikesInfo {
    #[pyo3(get)]
    data: Vec<LikeInfo>,
    #[pyo3(get)]
    paging: Option<meta::Paging>,
    #[pyo3(get)]
    summary: LikeSummary,
}


#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct PostInfo {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    message: Option<String>,
    #[pyo3(get)]
    created_time: String,
    #[pyo3(get)]
    story: Option<String>,
    #[pyo3(get)]
    likes: Option<BasicLikesInfo>,
    #[pyo3(get)]
    comments: Option<BasicCommentsInfo>,
}


#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct BasicPostsInfo {
    #[pyo3(get)]
    data: Vec<PostInfo>,
    #[pyo3(get)]
    paging: meta::Paging,

}

#[pyfunction]
pub fn get_facebook_page_info(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<BasicPageInfo>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::new();

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };


    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}?access_token={}", meta_version, page_id, access_token);
            println!("URL: {}", url);

            let res = client.get(&url).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;
            println!("Raw facebook page info text: {}", raw_text);
            let page_info: BasicPageInfo = serde_json::from_str(&raw_text)
                .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON response: {}", e)))?;
            
            Python::with_gil(|py| {
                Py::new(py, page_info).map_err(|e| PyErr::from(e))
            })
        

        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }

}

#[pyfunction]
pub fn get_facebook_page_followers(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<PageEngagement>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::new();

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };

    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}?fields=category,category_list,followers_count,fan_count,new_like_count,overall_star_rating,rating_count,talking_about_count&access_token={}", meta_version, page_id, access_token);
            println!("URL: {}", url);

            let res = client.get(&url).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;
            println!("Raw follower count text: {}", raw_text);
            let page_engagement: PageEngagement = serde_json::from_str(&raw_text)
                .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON response: {}", e)))?;
            
            Python::with_gil(|py| {
                Py::new(py, page_engagement).map_err(|e| PyErr::from(e))
            }) 

        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }

}

#[pyfunction]
pub fn get_facebook_page_posts(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<BasicPostsInfo>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::new();

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };

    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}/posts?&access_token={}", meta_version, page_id, access_token);
            println!("URL: {}", url);

            let res = client.get(&url).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;
            println!("Raw post text: {}", raw_text);

            let page_posts: BasicPostsInfo = serde_json::from_str(&raw_text)
                .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON response: {}", e)))?;
            
            Python::with_gil(|py| {
                Py::new(py, page_posts).map_err(|e| PyErr::from(e))
            }) 

        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }

}

#[pyfunction]
pub fn get_facebook_page_posts_with_summary(access_token: Option<String>, page_id: Option<String>, meta_version: Option<String>) -> PyResult<Py<BasicPostsInfo>> {
    let access_token = match access_token {
        Some(access_token) => access_token,
        None => auth::get_meta_access_token(None, None, None, None, None)?,
    };

    let client = Client::new();

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => auth::get_meta_version()?,
    };

    match page_id {
        Some(page_id) => {

            let url = format!("https://graph.facebook.com/{}/{}/posts?fields=id,message,created_time,likes.summary(true),comments.summary(true)&access_token={}", meta_version, page_id, access_token);
            println!("URL: {}", url);

            let res = client.get(&url).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;
            println!("Raw post text: {}", raw_text);
            let page_posts: BasicPostsInfo = serde_json::from_str(&raw_text)
            .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON response: {}", e)))?;
        
            Python::with_gil(|py| {
                Py::new(py, page_posts).map_err(|e| PyErr::from(e))
            }) 

        },
        None => Err(PyValueError::new_err("page_id must be set to the page id of the page to get info for"))?,
    }
}

#[pyfunction]
pub fn get_facebook_next_results(next: String) -> PyResult<String> {

    let client = Client::new();

    let res = client.get(&next).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
    let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;
    println!("Raw post text: {}", raw_text);

    Ok(raw_text)

}

#[pyfunction]
pub fn parse_next_results_to_basic_posts_info(raw_text: String) -> PyResult<Py<BasicPostsInfo>> {
    let page_posts: BasicPostsInfo = serde_json::from_str(&raw_text)
    .map_err(|e| PyValueError::new_err(format!("Failed to parse JSON response: {}", e)))?;
        
    Python::with_gil(|py| {
        Py::new(py, page_posts).map_err(|e| PyErr::from(e))
    }) 
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
            println!("URL: {}", url);
            let res = client.get(&url).send().map_err(|e| PyValueError::new_err(e.to_string()))?;
            let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;
            println!("Raw post reactions text: {}", raw_text);

            Ok(raw_text)
        },
        None => Err(PyValueError::new_err("Post ID is required")),
    }
            
}