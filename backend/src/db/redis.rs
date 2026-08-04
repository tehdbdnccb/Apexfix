use redis::Client;
use crate::config::Config;

pub fn init_redis(config: &Config) -> Result<Client, redis::RedisError> {
    Client::open(config.redis_url.as_str())
}