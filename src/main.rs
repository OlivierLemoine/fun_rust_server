use std::fs::read;

extern crate http_wrapper;
use http_wrapper::Server;

fn main() {
    Server::new("0.0.0.0:8080")
        .get("/", &|http: &mut http_wrapper::http::handler::Http| {
            let res = read("./static/index.html").unwrap();
            http.status = 200;
            http.write_response(res);
        })
        .post("/test", &|http: &mut http_wrapper::http::handler::Http| {
            http.status = 200;
            http.write_response(http.body.clone());
        })
        .statics("./static")
        .listen();
}
