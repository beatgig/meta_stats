use pyo3::prelude::*;

pub mod auth;
pub mod facebook;
pub mod instagram;
pub mod meta;

/// Python module definition
#[pymodule]
fn meta_stats(py: Python, m: &PyModule) -> PyResult<()> {
    let auth_module = PyModule::new(py, "auth")?;


    auth_module.add_function(wrap_pyfunction!(auth::get_meta_client_id, auth_module)?)?;
    auth_module.add_function(wrap_pyfunction!(auth::get_meta_client_secret, auth_module)?)?;
    auth_module.add_function(wrap_pyfunction!(auth::get_meta_access_token, auth_module)?)?;

    let facebook_module = PyModule::new(py, "facebook")?;


    facebook_module.add_function(wrap_pyfunction!(facebook::get_facebook_page_info, facebook_module)?)?;
    facebook_module.add_function(wrap_pyfunction!(facebook::get_facebook_page_followers, facebook_module)?)?;
    facebook_module.add_function(wrap_pyfunction!(facebook::get_facebook_page_posts, facebook_module)?)?;
    facebook_module.add_function(wrap_pyfunction!(facebook::get_facebook_page_posts_with_summary, facebook_module)?)?;
    facebook_module.add_function(wrap_pyfunction!(facebook::get_facebook_next_results, facebook_module)?)?;
    facebook_module.add_function(wrap_pyfunction!(facebook::get_facebook_post_interactions, facebook_module)?)?;
    facebook_module.add_function(wrap_pyfunction!(facebook::parse_next_results_to_basic_posts_info, facebook_module)?)?;

    let meta_module = PyModule::new(py, "meta")?;

    let instagram_module = PyModule::new(py, "instagram")?;

    instagram_module.add_function(wrap_pyfunction!(instagram::get_instagram_page_info, instagram_module)?)?;

    m.add_submodule(auth_module)?;
    m.add_submodule(facebook_module)?;
    m.add_submodule(meta_module)?;
    m.add_submodule(instagram_module)?;

    py.import("sys")?.getattr("modules")?.set_item("meta_stats.auth", auth_module)?;
    py.import("sys")?.getattr("modules")?.set_item("meta_stats.facebook", facebook_module)?;
    py.import("sys")?.getattr("modules")?.set_item("meta_stats.meta", meta_module)?;
    py.import("sys")?.getattr("modules")?.set_item("meta_stats.instagram", instagram_module)?;
    Ok(())
}
