use test_redis::client;
// use test_redis::server;

pub fn init_connection() {
    client::init_connection();
    // server::init_connection()
}

fn main() {
    println!("Hello, world!");
    init_connection()
}
