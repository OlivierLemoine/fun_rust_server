pub extern crate http;

use std::fs::read;

pub mod middlewares;
pub mod request_handler;

pub struct Server<'a> {
    server: http::Server,
    request_handlers: Vec<request_handler::RequestHandler<'a>>,
    middlewares: Vec<middlewares::Middleware<'a>>,
}

impl<'a> Server<'a> {
    pub fn new(addr: &str) -> Server {
        let server = http::Server::new(addr);

        Server {
            server,
            request_handlers: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn listen(mut self) -> Server<'a> {
        let handlers = &self.request_handlers;
        let middlewares = &self.middlewares;
        let handler = |http: &mut http::handler::Http| {
            for mid in middlewares.iter() {
                (mid.handler)(&mid.args, http);
            }
            for hand in handlers.iter() {
                if hand.request == http.request && hand.path == http.url.raw {
                    (hand.handler)(http);
                    return;
                }
            }
            http.status = 404;
            http.accept_types
                .insert(0, http::handler::AcceptTypes::TextHtml);
            http.write_response(Vec::from("File not found"));
        };

        self.server.listen(handler);
        self
    }

    pub fn get(mut self, path: &'a str, f: &'a Fn(&mut http::handler::Http)) -> Server<'a> {
        let req_handler = request_handler::RequestHandler {
            request: http::handler::Request::GET,
            path,
            handler: f,
        };

        self.request_handlers.push(req_handler);
        self
    }

    pub fn post(mut self, path: &'a str, f: &'a Fn(&mut http::handler::Http)) -> Server<'a> {
        let req_handler = request_handler::RequestHandler {
            request: http::handler::Request::POST,
            path,
            handler: f,
        };

        self.request_handlers.push(req_handler);
        self
    }

    pub fn add(
        mut self,
        args: Vec<&'a str>,
        f: &'a Fn(&Vec<&str>, &mut http::handler::Http),
    ) -> Server<'a> {
        let req_handler = middlewares::Middleware { args, handler: f };

        self.middlewares.push(req_handler);
        self
    }

    pub fn statics(self, path: &'a str) -> Server<'a> {
        self.add(
            vec![path],
            &|args: &Vec<&str>, http: &mut http::handler::Http| {
                let p = args.get(0).unwrap().clone();
                let r = http.url.raw.clone();
                let fp = format!("{}{}", p, r);
                match read(fp) {
                    Err(_e) => {}
                    Ok(v) => {
                        http.status = 200;
                        http.write_response(v);
                    }
                }
            },
        )
    }
}
