use pyo3::types::{PyDict};
use reqwest::blocking::Response;
use pyo3::{pyfunction, PyResult, Python};
use pyo3::exceptions::PyRuntimeError;
use reqwest::blocking::Client;
use chrono::Utc;
use crate::session::SESSION;
use pyo3::PyAny;

fn pydict_to_json(py_dict: &PyDict) -> PyResult<serde_json::Value> {
    use serde_json::Map;
    use serde_json::Value;

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
        Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(v).ok_or_else(|| PyRuntimeError::new_err("Invalid float"))?,
        ))
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
    let client = Client::builder()
    .danger_accept_invalid_certs(true)
    .build()
    .map_err(|e| PyRuntimeError::new_err(format!("Client error: {}", e)))?;

    let mut req = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        _ => return Err(PyRuntimeError::new_err("Unsupported HTTP method")),
    };

    req = req.header("Authorization", format!("Bearer {}", session.token));

    if let Some(h) = headers {
        for (key, value) in h {
            let key_str = key.extract::<String>()?;
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

    // Add JSON body
    if let Some(j) = json {
        let json_value = pydict_to_json(j)?;
        req = req.json(&json_value);
    }

    let res = req.send()
    .map_err(|e| PyRuntimeError::new_err(format!("Request failed: {}", e)))?;

    let status = res.status();
    let text = res.text()
    .map_err(|e| PyRuntimeError::new_err(format!("Read failed: {}", e)))?;

    if !status.is_success() {
        Err(PyRuntimeError::new_err(format!("Request failed: {} - {}", status, text)))
    } else {
        Ok(text)
    }
}

#[pyfunction]
pub fn get(py: Python, url: String, headers: Option<&PyDict>, params: Option<&PyDict>) -> PyResult<String> {
    make_request("GET", &url, headers, params, None)
}

#[pyfunction]
pub fn post(py: Python, url: String, headers: Option<&PyDict>, json: Option<&PyDict>) -> PyResult<String> {
    make_request("POST", &url, headers, None, json)
}
