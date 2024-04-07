use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
}

impl Counter {
    fn get_redis_url() -> String {
        let config = Config::new();

        config.map.get("REDIS_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned()
    }

    pub fn save(self) -> Result<(), redis::RedisError> {
        let serialised = serde_yaml::to_vec(&self).unwrap();

        let client = match redis::Client::open(Self::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error)
        };

        let mut conn = match client.get_connection() {
            Ok(conn) => conn,
            Err(error) => return Err(error)
        };

        match redis::cmd("SET").arg("COUNTER").arg(serialised).query::<Vec<u8>>(&mut conn) {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }

    }

    pub fn load() -> Result<Self, redis::RedisError> {
        let client = match redis::Client::open(Self::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error)
        };

        let mut conn = match client.get_connection() {
            Ok(conn) => conn,
            Err(error) => return Err(error)
        };

        let byte_data: Vec<u8> = match redis::cmd("GET").arg("COUNTER").query(&mut conn) {
            Ok(data) => data,
            Err(error) => return Err(error)
        };

        Ok(serde_yaml::from_slice(&byte_data).unwrap())

    }
}
