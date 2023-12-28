mod claims;
mod cli;
mod jaats;

use crate::cli::{get_args, Cli};
use crate::jaats::{jwt_decode, jwt_encode};

#[allow(clippy::print_stdout)]
fn main() {
    let args: Cli = get_args();

    match args {
        Cli::Encode {
            identifier,
            scopes,
            algorithm,
            secret,
            expiration_days,
        } => {
            let scope = scopes
                .split(',')
                .map(|pair: &str| {
                    let mut iter = pair.split('=');
                    (
                        iter.next().unwrap().to_string(),
                        iter.next().unwrap_or("").trim().to_string(),
                    )
                })
                .collect();

            match jwt_encode(&identifier, scope, algorithm, &secret, expiration_days) {
                Ok(encoded_token) => println!("{}", encoded_token),
                Err(err) => {
                    eprintln!("fucked up {:?}", err);
                    std::process::exit(1);
                }
            }
        }
        Cli::Decode {
            token,
            algorithm,
            secret,
        } => match jwt_decode(&token, algorithm, &secret) {
            Some(decoded_scope) => println!("{:?}", decoded_scope),
            None => {
                eprintln!("fucked up champ");
                std::process::exit(1);
            }
        },
    }
}
