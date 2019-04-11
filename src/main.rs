use std::fs::read;

extern crate http_wrapper;
use http_wrapper::Server;

fn main() {
    Server::new("0.0.0.0:8888")
        .get("/", &|http: &mut http_wrapper::http::handler::Http| {
            let res = read("./static/index.html").unwrap();
            http.status = 200;
            http.write_response(res);
        })
        .post("/test", &|http: &mut http_wrapper::http::handler::Http| {
            http.redirect_param(b"/test2".to_vec(), 307);
        })
        .get("/test2", &|http: &mut http_wrapper::http::handler::Http| {
            http.status = 200;
            http.write_response(http.body.clone());
        })
        .statics("./static")
        .listen();
}
