use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use std::collections::HashMap;
use crate::models::Claims;

pub fn jwt_encode(email: &str, scope: HashMap<String, String>, algorithm: Algorithm, secret: &str, expiration_days: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(email, scope, expiration_days);
    let header = Header::new(algorithm);
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    encode(&header, &claims, &encoding_key)
}


pub fn jwt_decode(encoded_token: &str, algorithm: Algorithm, secret: &str) -> Option<Claims> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(algorithm);
    match decode::<Claims>(encoded_token, &decoding_key, &validation) {
        Ok(token_data) => Some(token_data.claims),
        Err(_) => None,
    }
}