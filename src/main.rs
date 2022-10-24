use std::env;
use std::ptr::null;
use redis::Commands;
use dotenv::dotenv;

const DRIVER_VERSION: String = String::from("1.0.2");

static mut REDIS_HOST_NAME: String = String::from("127.0.0.1");
static mut REDIS_PASSWORD: String = String::from("foo");
static mut IS_TLS: String = String::from("redis");

fn main() {
    println!("Loading Rustandalone Redis Driver {:?}", DRIVER_VERSION.as_str());

    dotenv().expect(".env file not found");

    unsafe {
        REDIS_HOST_NAME = env::var("REDIS_HOST_NAME").expect("missing environment variable REDIS_HOSTNAME");
        REDIS_PASSWORD = env::var("REDIS_PASSWORD").expect("missing environment variable REDIS_PASSWORD");

        //Check if redis server needs a secure link for connection
        IS_TLS = match env::var("IS_TLS") {
            Ok(_) => "rediss",
            Err(_) => "redis",
        }.parse().unwrap();
    }
}

pub mod redis_driver {
    use std::env;
    use crate::{IS_TLS, REDIS_HOST_NAME, REDIS_PASSWORD};

    fn connect() -> redis::Connection {
        unsafe {
            let redis_conn_url = format!("{}://:{}@{}", IS_TLS, REDIS_PASSWORD, REDIS_HOST_NAME);

            return redis::Client::open(redis_conn_url)
                .expect("Invalid connection URL")
                .get_connection()
                .expect("Failed to connect to Redis")
        }
    }

}