use crate::http::method::Method;
use std::collections::HashMap;
use std::io::Read;
use std::net::{IpAddr, TcpStream};

pub struct Request {
    pub ip_addr: IpAddr,

    stream: TcpStream,
    content: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Self {
        let content = Self::read_content(stream).expect("Failed to read content");

        Self {
            stream: stream.try_clone().unwrap(),
            ip_addr: stream.peer_addr().expect("Failed to get IP").ip(),
            content: content,
        }
    }

    fn read_content(stream: &mut TcpStream) -> Result<String, std::io::Error> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }

    pub fn parse_content(self) -> (Method, String, HashMap<String, String>) {
        let lines: Vec<&str> = self.content.split("\r\n").collect();
        let request_line = lines[0];
        let mut parts = request_line.split_whitespace();

        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");
        let version = parts.next().unwrap_or("");

        println!("Method: {}", method);
        println!("Path: {}", path);
        println!("Version: {}", version);

        // let headers = HashMap::new();

        for line in &lines[1..] {
            if line.is_empty() {
                break;
            }
            println!("Header: {}", line);

            let t = line.split(": ").collect::<Vec<&str>>();
            let value = t.get(1).unwrap_or(&"").to_string();
            let key = t.get(0).unwrap_or(&"").to_string();

            print!("{} : {}", key, value);

            // headers.insert(line.to_string(), line.to_string());
        }

        (Method::Get, String::from(path), HashMap::new())
    }
}
