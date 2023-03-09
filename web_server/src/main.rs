/* made following a blog */ 

use std::net::TcpListener; 
use std::net::TcpStream; 
use std::io::prelude::*;
use std::fs;

fn main() {

    const host: &str = "127.0.0.1"; 
    const port: &str = "3000";

    let endp : String = host.to_owned() + ":" + port;
    let Listener = TcpListener::bind(endp).unwrap();

    println!("Web server up at port {}", port);

    for stream in Listener.incoming(){
        let _stream = stream.unwrap();

        handle_connection(_stream);

    }
}

fn handle_connection(mut Stream: TcpStream){
    let mut buffer = [0; 1024]; 
    Stream.read(&mut buffer).unwrap();

    let response_page = fs::read_to_string("index.html").unwrap();
    
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_page.len(),
        response_page
    );

    Stream.write(response.as_bytes()).unwrap();
    Stream.flush().unwrap();
}
