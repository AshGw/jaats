use structopt::StructOpt;
use jsonwebtoken::Algorithm;

#[derive(StructOpt, Debug)]
pub enum Cli {
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

pub fn get_args() -> Cli{
    return Cli::from_args();
}