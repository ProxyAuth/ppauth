use pyo3::types::PyDict;
use pyo3::{pyfunction, PyResult, Python};
use pyo3::exceptions::PyRuntimeError;
use pyo3::PyAny;
use reqwest::blocking::{Client, Response};
use chrono::Utc;
use std::thread::sleep;
use std::time::Duration;

use crate::session::SESSION;

fn pydict_to_json(py_dict: &PyDict) -> PyResult<serde_json::Value> {
    use serde_json::{Map, Value};

    let mut map = Map::new();
    for (k, v) in py_dict {
        let key = k.extract::<String>()?;
        let value = python_value_to_json(v)?;
        map.insert(key, value);
    }
    Ok(Value::Object(map))
}

fn python_value_to_json(obj: &PyAny) -> PyResult<serde_json::Value> {
    if let Ok(v) = obj.extract::<bool>() {
        Ok(serde_json::Value::Bool(v))
    } else if let Ok(v) = obj.extract::<i64>() {
        Ok(serde_json::Value::Number(v.into()))
    } else if let Ok(v) = obj.extract::<f64>() {
        Ok(serde_json::Number::from_f64(v)
        .map(serde_json::Value::Number)
        .ok_or_else(|| PyRuntimeError::new_err("Invalid float"))?)
    } else if let Ok(v) = obj.extract::<String>() {
        Ok(serde_json::Value::String(v))
    } else if obj.downcast::<PyDict>().is_ok() {
        pydict_to_json(obj.downcast::<PyDict>().unwrap())
    } else if obj.downcast::<pyo3::types::PyList>().is_ok() {
        let list = obj.downcast::<pyo3::types::PyList>().unwrap();
        let mut result = Vec::new();
        for item in list {
            result.push(python_value_to_json(item)?);
        }
        Ok(serde_json::Value::Array(result))
    } else {
        Err(PyRuntimeError::new_err("Unsupported JSON value"))
    }
}

fn make_request(
    method: &str,
    path: &str,
    headers: Option<&PyDict>,
    params: Option<&PyDict>,
    json: Option<&PyDict>,
    timeout: Option<u64>,
    verify: Option<bool>,
    retry: Option<u32>,
) -> PyResult<String> {
    let session = {
        let stored = SESSION.lock().unwrap();
        stored.clone().ok_or_else(|| PyRuntimeError::new_err("Not authenticated. Call auth() first."))?
    };

    let now = Utc::now().naive_utc();
    if session.expires_at <= now {
        return Err(PyRuntimeError::new_err("Token expired. Call token(renew=True)."));
    }

    let url = format!("https://{}:{}/{}", session.host, session.port, path.trim_start_matches('/'));

    let verify_tls = verify.unwrap_or(true);
    let timeout_duration = Duration::from_secs(timeout.unwrap_or(10));
    let max_retries = retry.unwrap_or(0);

    let client = Client::builder()
    .timeout(timeout_duration)
    .danger_accept_invalid_certs(!verify_tls)
    .build()
    .map_err(|e| PyRuntimeError::new_err(format!("Client error: {}", e)))?;

    let mut last_err = None;

    for attempt in 0..=max_retries {
        let mut req = match method {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            _ => return Err(PyRuntimeError::new_err("Unsupported HTTP method")),
        };

        req = req.header("Authorization", format!("Bearer {}", session.token));

        if let Some(h) = headers {
            for (key, value) in h {
                let key_str = key.extract::<String>()?;
                if key_str.to_lowercase() == "authorization" {
                    continue;
                }
                let val_str = value.extract::<String>()?;
                req = req.header(key_str, val_str);
            }
        }


        if let Some(p) = params {
            let mut query = vec![];
            for (key, value) in p {
                let key_str = key.extract::<String>()?;
                let val_str = value.extract::<String>()?;
                query.push((key_str, val_str));
            }
            req = req.query(&query);
        }

        if let Some(j) = json {
            let json_value = pydict_to_json(j)?;
            req = req.json(&json_value);
        }

        match req.send() {
            Ok(res) => {
                let status = res.status();
                let text = res.text().map_err(|e| {
                    PyRuntimeError::new_err(format!("Read failed: {}", e))
                })?;

                if status.is_success() {
                    return Ok(text);
                } else if status.is_server_error() && attempt < max_retries {
                    sleep(Duration::from_millis(5));
                    continue;
                } else {
                    return Err(PyRuntimeError::new_err(format!("Request failed: {} - {}", status, text)));
                }
            }
            Err(err) => {
                last_err = Some(err);
                if attempt < max_retries {
                    sleep(Duration::from_millis(5));
                    continue;
                }
            }
        }
    }

    Err(PyRuntimeError::new_err(format!(
        "Request failed after {} attempt(s): {:?}",
        max_retries + 1,
        last_err.map(|e| e.to_string()).unwrap_or_else(|| "Unknown error".to_string())
    )))
}

#[pyfunction(
signature = (
    path,
    headers = None,
    params = None,
    timeout = None,
    verify = None,
    retry = None
)
)]
pub fn get(
    _py: Python,
    path: String,
    headers: Option<&PyDict>,
    params: Option<&PyDict>,
    timeout: Option<u64>,
    verify: Option<bool>,
    retry: Option<u32>,
) -> PyResult<String> {
    make_request("GET", &path, headers, params, None, timeout, verify, retry)
}

#[pyfunction(
signature = (
    path,
    headers = None,
    json = None,
    timeout = None,
    verify = None,
    retry = None
)
)]
pub fn post(
    _py: Python,
    path: String,
    headers: Option<&PyDict>,
    json: Option<&PyDict>,
    timeout: Option<u64>,
    verify: Option<bool>,
    retry: Option<u32>,
) -> PyResult<String> {
    make_request("POST", &path, headers, None, json, timeout, verify, retry)
}
