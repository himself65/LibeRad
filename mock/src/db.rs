pub fn connect_redis() -> redis::Connection {
    let client = redis::Client::open("redis://123456@127.0.0.1:6379")
        .expect("Get redis client error");
    client.get_connection().expect("Get connection error")
}