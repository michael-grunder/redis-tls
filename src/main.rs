fn main() {
    let port = std::env::args().nth(1).unwrap_or("56443".to_string());

    let uri = format!("rediss://localhost:{}#insecure", port);
    let client = redis::Client::open(&*uri).expect("Can't create Redis client");

    let cmd = redis::cmd("PING");

    let mut con = client.get_connection().expect("Can't get Redis connection");

    // Of course this doesn't work, but I can't figure out how to specify certs/keys
    let resp: String = cmd.query(&mut con).expect("Can't query Redis");

    println!("PING: {}", resp);
}
