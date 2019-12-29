pub fn connect_redis() -> redis::Client {
    redis::Client::open("redis://localhost:6379")
        .expect("Get redis client error")
}