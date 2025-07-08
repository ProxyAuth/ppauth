use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use chrono::{Utc, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyRuntimeError;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone)]
struct AuthResponse {
    token: String,
    expires_at: String,
}

#[derive(Clone)]
struct Session {
    token: String,
    expires_at: NaiveDateTime,
    host: String,
    port: u16,
    auth: AuthRequest,
    timezone: String,
}

static SESSION: Lazy<Arc<Mutex<Option<Session>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

#[pyfunction]
fn auth(
    host: String,
    port: u16,
    username: String,
    password: String,
    timezone: Option<String>,
) -> PyResult<()> {
    let client = Client::builder()
    .danger_accept_invalid_certs(true)
    .build()
    .map_err(|e| PyRuntimeError::new_err(format!("Client error: {}", e)))?;

    let full_url = format!("https://{}:{}/auth", host.trim_end_matches('/'), port);
    let body = AuthRequest { username, password };

    let res = client
    .post(&full_url)
    .json(&body)
    .send()
    .map_err(|e| PyRuntimeError::new_err(format!("Request failed: {}", e)))?;

    if !res.status().is_success() {
        return Err(PyRuntimeError::new_err(format!(
            "Authentication failed with status code {}",
            res.status()
        )));
    }

    let text = res
    .text()
    .map_err(|e| PyRuntimeError::new_err(format!("Response read failed: {}", e)))?;

    let auth_response: AuthResponse = serde_json::from_str(&text)
    .map_err(|e| PyRuntimeError::new_err(format!("Invalid JSON: {}", e)))?;

    let expiry_naive = NaiveDateTime::parse_from_str(&auth_response.expires_at, "%Y-%m-%d %H:%M:%S")
    .map_err(|e| PyRuntimeError::new_err(format!("Invalid expires_at format: {}", e)))?;

    let tz_name = timezone.unwrap_or_else(|| "UTC".to_string());
    let tz: Tz = tz_name
    .parse()
    .map_err(|_| PyRuntimeError::new_err("Invalid timezone provided"))?;

    let expiry_local = tz
    .from_local_datetime(&expiry_naive)
    .single()
    .ok_or_else(|| PyRuntimeError::new_err("Ambiguous or invalid datetime in timezone"))?;

    let expiry_utc = expiry_local.naive_utc();

    let mut stored = SESSION.lock().unwrap();
    *stored = Some(Session {
        token: auth_response.token.clone(),
        expires_at: expiry_utc,
        host,
        port,
        auth: body,
        timezone: tz_name,
    });

    Ok(())
}

#[pyfunction]
fn is_logged() -> bool {
    let stored = SESSION.lock().unwrap();
    if let Some(session) = &*stored {
        let now = Utc::now().naive_utc();
        return session.expires_at > now;
    }
    false
}

#[pyfunction]
fn token(renew: Option<bool>) -> PyResult<String> {
    let renew = renew.unwrap_or(false);
    let now = Utc::now().naive_utc();

    {
        let stored = SESSION.lock().unwrap();
        if let Some(session) = &*stored {
            if session.expires_at > now {
                return Ok(session.token.clone());
            }
        }
    }

    if !renew {
        return Err(PyRuntimeError::new_err("Token expired"));
    }

    let (host, port, username, password, timezone) = {
        let stored = SESSION.lock().unwrap();
        if let Some(session) = &*stored {
            (
             session.host.clone(),
             session.port,
             session.auth.username.clone(),
             session.auth.password.clone(),
             Some(session.timezone.clone()),
            )
        } else {
            return Err(PyRuntimeError::new_err("No token stored. Call auth() first."));
        }
    };

    auth(host, port, username, password, timezone)?;
    let stored = SESSION.lock().unwrap();
    if let Some(session) = &*stored {
        Ok(session.token.clone())
    } else {
        Err(PyRuntimeError::new_err("Token renewal failed."))
    }
}

#[pyfunction]
fn lease_token() -> PyResult<i64> {
    let stored = SESSION.lock().unwrap();
    if let Some(session) = &*stored {
        let now = Utc::now().naive_utc();
        let remaining = (session.expires_at - now).num_seconds();
        Ok(remaining.max(0))
    } else {
        Err(PyRuntimeError::new_err("No token stored. Call auth() first."))
    }
}

#[pymodule]
fn pyproxyauth(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(auth, m)?)?;
    m.add_function(wrap_pyfunction!(token, m)?)?;
    m.add_function(wrap_pyfunction!(lease_token, m)?)?;
    m.add_function(wrap_pyfunction!(is_logged, m)?)?;
    Ok(())
}
