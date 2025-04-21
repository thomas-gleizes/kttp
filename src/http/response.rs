use crate::http::headers::Headers;
use crate::http::status_code::StatusCode;
use std::io::Write;
use std::net::TcpStream;

pub struct Response {
    status: StatusCode,
    headers: Headers,

    stream: TcpStream,
}

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            status: StatusCode::new(200).unwrap(),
            headers: Headers::new(),
            stream,
        }
    }

    pub fn status(&mut self, status: StatusCode) {
        self.status = status;
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.add(key, value);
    }

    pub fn send(&mut self, content: String) {
        let response = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status.to_u16(),
            self.status.message(),
            self.headers.to_string(),
            content
        );

        self.stream.write_all(response.as_bytes()).unwrap();
    }
}
