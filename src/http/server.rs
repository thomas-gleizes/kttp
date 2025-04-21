use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::Response;
use std::collections::HashMap;
use std::io;
use std::net::{TcpListener, ToSocketAddrs};

type Handler = fn(&HttpContext) -> Response;

struct HttpContext {
    pub request: Request,
    pub response: Response,
}

struct Route {
    method: Method,
    handler: Handler,
}

pub struct Server {
    tcp_listener: io::Result<TcpListener>,
    is_listening: bool,

    routes: HashMap<String, Vec<Route>>,
}

impl Server {
    pub fn new<A: ToSocketAddrs>(address: A) -> Self {
        Self {
            tcp_listener: TcpListener::bind(address),
            is_listening: false,

            routes: HashMap::new(),
        }
    }

    pub fn get(self, path: &str, handler: Handler) {
        self.route(path, Method::Get, handler);
    }

    pub fn post(self, path: &str, handler: Handler) {
        self.route(path, Method::Post, handler);
    }

    pub fn put(self, path: &str, handler: Handler) {
        self.route(path, Method::Put, handler);
    }

    pub fn delete(self, path: &str, handler: Handler) {
        self.route(path, Method::Delete, handler);
    }

    pub fn patch(self, path: &str, handler: Handler) {
        self.route(path, Method::Patch, handler);
    }

    pub fn route(mut self, path: &str, method: Method, handler: Handler) {
        // register the route
        if (self.routes.get(path).is_none()) {
            self.routes.insert(path.to_owned(), Vec::new());
        }
        
        
        
    }

    pub fn listen(mut self) {
        self.is_listening = true;
        let listener = self.tcp_listener.expect("Failed to bind to address");

        for stream in listener.incoming() {
            match stream {
                Ok(_) => {
                    // handle_client(stream);
                    println!("Client connecté");
                }
                Err(e) => eprintln!("Erreur lors de l'acceptation de la connexion : {}", e),
            }
        }
    }
}
