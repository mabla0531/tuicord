use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // login test
    // init server
    let server = TcpListener::bind("0.0.0.0:7463").unwrap();

    open::that(format!("https://discord.com/oauth2/authorize?client_id={}&response_type=code&redirect_uri=http%3A%2F%2F0.0.0.0%3A7463&scope=messages.read", fs::read_to_string(".env").unwrap())).unwrap();

    // receive on server
    for stream in server.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|res| res.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Request Received: \n\n{http_request:#?}");

        stream.write_all("HTTP/1.1 200 OK".as_bytes()).unwrap();
        // TODO write a confirmation or declination HTTP response with an HTML body that looks pretty (the user will see this in their browser after auth)
    }
    // close server

    // hold auth token
}
