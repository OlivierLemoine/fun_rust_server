use super::http::handler;

pub struct RequestHandler<'a>{
    pub request: handler::Request,
    pub path: &'a str,
    pub handler: &'a Fn(&mut handler::Http),
}