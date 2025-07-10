use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static ALLOW_AUTO_RENEW: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
