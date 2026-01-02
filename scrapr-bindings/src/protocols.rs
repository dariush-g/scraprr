use crate::request::*;
use pyo3::prelude::*;

#[pyfunction]
pub fn fetch_http(host: &str, path: &str) -> PyResult<String> {
    scrapr_core::fetch_http(host, path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn fetch_http_with_options(
    host: &str,
    path: &str,
    options: RequestOptions,
) -> PyResult<String> {
    let core_options = scrapr_core::RequestOptions {
        headers: options.headers,
        cookies: options.cookies,
        query: options.query,
    };

    scrapr_core::fetch_http_with_options(host, path, core_options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn extract_attribute(text: &str, tag: &str, attr: &str) -> PyResult<Vec<String>> {
    scrapr_core::extract_attribute(text, tag, attr)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn extract_links(text: &str) -> PyResult<Vec<String>> {
    extract_attribute(text, "a", "href")
}

// pub fn fetch_http_(host: &str, path: &str) -> PyResult<String> {
//     let mut stream = TcpStream::connect((host, 80))
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;
//     stream.set_write_timeout(Some(std::time::Duration::from_secs(5)))
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");

//     stream.write_all(request.as_bytes())
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     let mut response = String::new();
//     stream.read_to_string(&mut response)
//         .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

//     // Check HTTP status
//     let status_line = response.lines().next().unwrap_or_default();
//     if !status_line.contains("200 OK") {
//         return Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!("HTTP error: {}", status_line)));
//     }

//     if let Some(body) = response.split("\r\n\r\n").nth(1) {
//         Ok(body.to_string())
//     } else {
//         Err(PyErr::new::<pyo3::exceptions::PyException, _>("No body found"))
//     }
// }

#[pyfunction]
pub fn extract_tag(text: &str, tag: &str) -> PyResult<Vec<String>> {
    scrapr_core::extract_tag(text, tag)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn fetch_https(host: &str, path: &str) -> PyResult<String> {
    scrapr_core::fetch_https(host, path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn fetch_https_with_options(
    host: &str,
    path: &str,
    options: RequestOptions,
) -> PyResult<String> {
    let core_options = scrapr_core::RequestOptions {
        headers: options.headers,
        cookies: options.cookies,
        query: options.query,
    };

    scrapr_core::fetch_https_with_options(host, path, core_options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
