use std::sync::{Arc, Mutex};
use chrono::{NaiveDateTime};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Session {
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub host: String,
    pub port: u16,
    pub auth: AuthRequest,
    pub timezone: String,
}

#[derive(Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: String,
}

pub static SESSION: Lazy<Arc<Mutex<Option<Session>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
