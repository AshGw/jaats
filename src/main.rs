
mod models;
mod jaats;
mod cli;
use structopt::StructOpt;
use std::collections::HashMap;
use crate::jaats::{jwt_encode, jwt_decode};
use crate::cli::Cli;

fn main() {
    let args: Cli = Cli::from_args();

    match args {
        Cli::Encode { email, scopes, algorithm, secret, expiration_days } => {
            let scope: HashMap<String, String> = scopes
                .split(',')
                .map(|pair: &str| {
                    let mut iter: std::str::Split<'_, char> = pair.split('=');
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