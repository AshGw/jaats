use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub scope: HashMap<String, String>,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(email: &str, scope: HashMap<String, String>, expiration_days: i64) -> Self {
        let current_time = Utc::now().timestamp();
        Claims {
            email: email.to_string(),
            scope,
            iat: current_time,
            exp: current_time + expiration_days * 24 * 3600,
        }
    }
}