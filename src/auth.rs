use pyo3::prelude::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use pyo3::exceptions::PyValueError;

#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponse {
    access_token: String,
}

#[pyfunction]
pub fn get_meta_client_id() -> PyResult<String> {
    dotenv().ok();

    match env::var("META_CLIENT_ID") {
        Ok(client_id) => Ok(client_id),
        Err(_) => Err(PyValueError::new_err("META_CLIENT_ID not set in environment")),
    }
}

#[pyfunction]
pub fn get_meta_client_secret() -> PyResult<String> {
    dotenv().ok();

    match env::var("META_CLIENT_SECRET") {
        Ok(client_secret) => Ok(client_secret),
        Err(_) => Err(PyValueError::new_err("META_CLIENT_SECRET not set in environment")),
    }
}

#[pyfunction]
pub fn get_meta_version() -> PyResult<String> {
    dotenv().ok();

    match env::var("META_VERSION") {
        Ok(version) => Ok(version),
        Err(_) => Err(PyValueError::new_err("META_VERSION not set in environment")),
    }
}

#[pyfunction]
pub fn get_meta_access_token(endpoint_url: Option<String>, client_id: Option<String>, client_secret: Option<String>, grant_type: Option<String>, meta_version: Option<String>) -> PyResult<String> {
    dotenv().ok();
    let client_id = match client_id {
        Some(client_id) => client_id,
        None => get_meta_client_id()?,
    };

    let client_secret = match client_secret {
        Some(client_secret) => client_secret,
        None => get_meta_client_secret()?,
    };

    let grant_type = match grant_type {
        Some(grant_type) => grant_type,
        None => "client_credentials".to_string(),
    };

    let meta_version = match meta_version {
        Some(meta_version) => meta_version,
        None => get_meta_version()?,
    };

    let formatted_version = if meta_version.starts_with("v") {
        meta_version
    } else {
        format!("v{}", meta_version)
    };

    let endpoint_url = match endpoint_url {
        Some(endpoint_url) => endpoint_url,
        None => format!("https://graph.facebook.com/{}/oauth/access_token", formatted_version),
    };

    let url = format!(
        "{}?client_id={}&client_secret={}&grant_type={}",
        endpoint_url, client_id, client_secret, grant_type
    );

    let client = Client::new();
    let res = client.get(&url).send().map_err(|e| PyValueError::new_err(e.to_string()))?;

    let raw_text = res.text().map_err(|e| PyValueError::new_err(e.to_string()))?;


    let response: Result<AccessTokenResponse, serde_json::Error> = serde_json::from_str(&raw_text);

    match response {
        Ok(response) => Ok(response.access_token),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}