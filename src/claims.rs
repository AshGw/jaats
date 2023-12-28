use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Scopes = HashMap<String, String>; // prolly change it later

#[derive(Debug, Serialize, Deserialize)]

pub struct Claims {
    pub identifier: String,
    pub scope: Scopes,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(id: &str, scope: Scopes, expiration_days: i64) -> Self {
        let current_time: i64 = Utc::now().timestamp();
        Claims {
            identifier: id.to_string(),
            scope,
            iat: current_time,
            exp: current_time + expiration_days * 24 * 3600,
        }
    }
}
