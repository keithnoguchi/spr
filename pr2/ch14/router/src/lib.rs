//! Simple Router Exercise
use std::collections::HashMap;
use std::fmt::{self, Debug};

#[derive(Default)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Default)]
pub struct Response {
    pub code: u32,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn is_error(&self) -> bool {
        self.code == 500
    }

    fn error() -> Self {
        Self {
            code: 500,
            ..Self::default()
        }
    }
}

// Callback as Trait Object.
type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

#[derive(Default)]
pub struct Router {
    routes: HashMap<String, BoxedCallback>,
}

impl Debug for Router {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Router")
            .field("routes.len()", &self.routes.len())
            .finish()
    }
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<P, C>(&mut self, path: P, callback: C) -> &mut Self
    where
        P: ToString,
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(callback));
        self
    }

    pub fn route(&self, req: &Request) -> Response {
        match self.routes.get(&req.url) {
            None => Response::error(),
            Some(callback) => callback(req),
        }
    }
}
