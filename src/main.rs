mod http;

use crate::http::server::Server;
use http::request::Request;
use std::io::{Read, Write};
use std::net::{TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Tampon pour lire les données

    let request = Request::new(&mut stream);

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Requête reçue :\n{}", request);

            // Réponse simple
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Erreur lors de la lecture : {}", e),
    }
}

fn main() {
    let server = Server::new("0.0.0.0:80");

    // server.get("/", |mut context| {
    //     context.response.send("<h1>Hello, world!</h1>".to_string());
    // });

    server.listen()
}
