use crate::claims::{Claims, Scopes};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub fn jwt_encode(
    email: &str,
    scope: Scopes,
    algorithm: Algorithm,
    secret: &str,
    expiration_days: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims: Claims = Claims::new(email, scope, expiration_days);
    let header: Header = Header::new(algorithm);
    let encoding_key: EncodingKey = EncodingKey::from_secret(secret.as_bytes());
    encode(&header, &claims, &encoding_key)
}

pub fn jwt_decode(encoded_token: &str, algorithm: Algorithm, secret: &str) -> Option<Claims> {
    let decoding_key: DecodingKey = DecodingKey::from_secret(secret.as_bytes());
    let validation: Validation = Validation::new(algorithm);
    match decode::<Claims>(encoded_token, &decoding_key, &validation) {
        Ok(token_data) => Some(token_data.claims),
        Err(_) => None, // hushhh
    }
}
