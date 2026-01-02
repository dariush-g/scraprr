use crate::request::RequestOptions;
use pyo3::{PyErr, PyResult, pyfunction};

#[pyfunction]
pub fn fetch_url(url: &str) -> PyResult<String> {
    scrapr_core::fetch_url(url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn fetch_url_with_options(url: &str, opts: RequestOptions) -> PyResult<String> {
    let core_options = scrapr_core::RequestOptions {
        headers: opts.headers,
        cookies: opts.cookies,
        query: opts.query,
    };

    scrapr_core::fetch_url_with_options(url, core_options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
