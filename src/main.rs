mod http;

use http::request::Request;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Tampon pour lire les données

    let request = Request::new(&mut stream);

    request.parse_content();

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

fn main() -> std::io::Result<()> {
    println!("Serveur HTTP en écoute sur le port 80...");
    let listener = TcpListener::bind("0.0.0.0:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
