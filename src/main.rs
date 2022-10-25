use std::borrow::Borrow;
use std::env;
use redis::{Commands, Connection, ConnectionLike};
use dotenv::dotenv;

const DRIVER_VERSION: &str = "1.0.2";

static mut REDIS_HOST_NAME: String = String::new();
static mut REDIS_PASSWORD: String = String::new();
static mut IS_TLS: String = String::new();

fn main() {
    println!("Loading Rustandalone Redis Driver {:?}", DRIVER_VERSION);

    dotenv().expect(".env file not found");

    unsafe {
        REDIS_HOST_NAME = env::var("REDIS_HOST_NAME").expect("missing environment variable REDIS_HOST_NAME");
        REDIS_PASSWORD = env::var("REDIS_PASSWORD").expect("missing environment variable REDIS_PASSWORD");
        //Check if redis server needs a secure link for connection
        IS_TLS = match env::var("IS_TLS") {
            Ok(_) => "rediss",
            Err(_) => "redis",
        }.to_string();
    }

    test_connection();
}


fn test_connection() {
    let connection:Connection = redis_driver::connect();

    if connection.is_open() {
        println!("The connection to redis was established successfully")
    }
}

pub mod redis_driver {
    use std::{fmt, io};
    use std::fmt::{Display, Formatter};
    use std::task::ready;
    use redis::{Client, Commands, Connection, ConnectionLike, ErrorKind, RedisError, RedisResult};
    use crate::{IS_TLS, REDIS_HOST_NAME, REDIS_PASSWORD};

    pub fn connect() -> Connection{
        unsafe {
            let redis_conn_url = format!("{}://:{}@{}", IS_TLS, REDIS_PASSWORD, REDIS_HOST_NAME);

            return Client::open(redis_conn_url)
                .expect(RRedisError::InvalidURL.get())
                .get_connection()
                .expect(RRedisError::ConnectionClosed.get());
        }
    }

    type RedisResultValue<T> = Result<T, RedisResultError>;

    pub enum RRedisError {
        NullPointer,
        ConnectionClosed,
        Overflow,
        Other,
        CannotGet,
        InvalidURL
    }

    pub struct RedisResultError {
        error: RRedisError,
    }

    impl RRedisError {
        fn get(&self) -> &str {
            match self {
                RRedisError::NullPointer => "unable to retrieve value",
                RRedisError::ConnectionClosed => "connection could not be established with redis",
                RRedisError::Overflow => "your redis key value is overflow",
                RRedisError::Other => "An error occurred during your request",
                RRedisError::CannotGet => "impossible to retrieve the contents of the key, maybe it does not exist",
                RRedisError::InvalidURL => "unable to establish connection to redis, please check your environment variables"
            }
        }

    }

    impl fmt::Debug for RedisResultError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let error_message: String = self.error.get().to_string();

            return write!(f, "{:?}", error_message);
        }
    }

    pub fn get<T>(key: &str) -> RedisResultValue<T> {
        let mut conn = connect();

        if conn.is_open() {
           /* let val: T = conn
                .get(key).ok_or(RedisResultError { error: RRedisError::CannotGet });*/
        }

        return Err(RedisResultError {
            error: RRedisError::ConnectionClosed
        });
    }

}