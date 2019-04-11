use super::http::handler;

pub struct Middleware<'a> {
    pub args: Vec<&'a str>,
    pub handler: &'a Fn(&Vec<&str>, &mut handler::Http),
}
