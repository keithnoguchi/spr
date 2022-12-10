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

#[derive(Default)]
pub struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>,
}

impl Debug for FnPointerRouter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FnPointerRouter")
            .field("routes.len()", &self.routes.len())
            .finish()
    }
}

impl FnPointerRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<P>(mut self, path: P, callback: fn(&Request) -> Response) -> Self
    where
        P: ToString,
    {
        self.routes.insert(path.to_string(), callback);
        self
    }

    pub fn route(&self, req: &Request) -> Response {
        match self.routes.get(&req.url) {
            None => Response::error(),
            Some(callback) => callback(req),
        }
    }
}

// Callback as Trait Object.
type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

#[derive(Default)]
pub struct ClosureRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl Debug for ClosureRouter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ClosureRouter")
            .field("routes.len()", &self.routes.len())
            .finish()
    }
}

impl ClosureRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<P, C>(mut self, path: P, callback: C) -> Self
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
