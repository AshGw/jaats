use std::collections::HashMap;
use structopt::StructOpt;
use crate::jaats::{jwt_encode, jwt_decode};
use jsonwebtoken::Algorithm;

mod models;
mod jaats;

#[derive(StructOpt, Debug)]
enum Cli {
    #[structopt(name = "encode")]
    Encode {
        #[structopt(long = "email")]
        email: String,

        #[structopt(long = "scopes")]
        scopes: String,

        #[structopt(long = "algorithm", default_value = "HS256")]
        algorithm: Algorithm,

        #[structopt(long = "secret")]
        secret: String,

        #[structopt(long = "expiration-days", default_value = "30")]
        expiration_days: i64,
    },
    #[structopt(name = "decode")]
    Decode {
        #[structopt(long = "token")]
        token: String,

        #[structopt(long = "algorithm", default_value = "HS256")]
        algorithm: Algorithm,

        #[structopt(long = "secret")]
        secret: String,
    },
}


fn main() {
    let args = Cli::from_args();

    match args {
        Cli::Encode { email, scopes, algorithm, secret, expiration_days } => {
            let scope: HashMap<String, String> = scopes
                .split(',')
                .map(|pair| {
                    let mut iter = pair.split('=');
                    (iter.next().unwrap().to_string(), iter.next().unwrap_or("").trim().to_string())
                })
                .collect();

            match jwt_encode(&email, scope, algorithm, &secret, expiration_days) {
                Ok(encoded_token) => println!("{}", encoded_token),
                Err(err) => {
                    eprintln!("Failed, error: {:?}", err);
                    std::process::exit(1);
                }
            }
        }
        Cli::Decode { token, algorithm, secret } => {
            match jwt_decode(&token, algorithm, &secret) {
                Some(decoded_scope) => println!("{:?}", decoded_scope),
                None => {
                    eprintln!("Failed");
                    std::process::exit(1);
                }
            }
        }
    }
}