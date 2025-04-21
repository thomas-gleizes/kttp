use crate::http::headers::Headers;
use crate::http::method::Method;
use crate::http::status_code::StatusCode;
use std::collections::HashMap;
use std::io::Read;
use std::net::{IpAddr, TcpStream};

pub struct Request {
    pub ip_addr: IpAddr,
    pub headers: Headers,
    pub method: Method,
    pub path: String,

    stream: TcpStream,
    content: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Self {
        let content = Self::read_content(stream).expect("Failed to read content");
        let (method, path, headers) = Self::parse_content(content.clone());

        Self {
            stream: stream.try_clone().unwrap(),
            ip_addr: stream.peer_addr().expect("Failed to get IP").ip(),
            content,
            headers: Headers::new(),
            method,
            path,
        }
    }

    fn read_content(stream: &mut TcpStream) -> Result<String, std::io::Error> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }

    fn parse_content(content: String) -> (Method, String, Headers) {
        let lines: Vec<&str> = content.split("\r\n").collect();
        let request_line = lines[0];
        let mut parts = request_line.split_whitespace();

        let method = parts.next().unwrap_or("");
        let path = parts.next().unwrap_or("");
        let version = parts.next().unwrap_or("");

        println!("Method: {}", method);
        println!("Path: {}", path);
        println!("Version: {}", version);

        let mut headers = Headers::new();

        for line in &lines[1..] {
            if line.is_empty() {
                break;
            }
            println!("Header: {}", line);

            let t = line.split(": ").collect::<Vec<&str>>();

            headers.add(t[0].to_string(), t[1].to_string());
        }

        (Method::Get, String::from(path), headers)
    }
}
