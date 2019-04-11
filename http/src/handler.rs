use super::url;
use std::io::prelude::{Read, Write};
use std::net::TcpStream;

#[derive(PartialEq)]
pub enum Request {
    GET,
    POST,
}

pub enum AcceptTypes {
    TextHtml,
    ImageWebp,
    WildcardWildcard,
}

pub enum Connection {
    Close,
    KeepAlive,
}

pub struct Http<'a> {
    stream: &'a TcpStream,

    pub request: Request,
    pub url: url::Url,

    pub connection: Connection,

    pub status: u32,

    pub accept_types: Vec<AcceptTypes>,

    pub body: Vec<u8>,
}

impl<'a> Http<'a> {
    pub fn parse(stream: &mut TcpStream) -> Result<Http, std::io::Error> {
        let mut request = Request::GET;
        let mut url = String::from("");
        let mut accept_types: Vec<AcceptTypes> = vec![];
        let mut connection = Connection::KeepAlive;

        let mut body_value: Vec<u8> = Vec::new();

        let mut body: u8 = 0;
        let mut body_len: u32 = 0;

        loop {
            let mut buffer = vec![0; 1024];

            let s = match stream.read(&mut buffer[..]) {
                Err(e) => return Err(e),
                Ok(s) => s,
            };
            if s == 0 {
                break;
            }

            for line in String::from_utf8(buffer).unwrap().split("\r\n") {
                let mut line_split = line.split(" ");

                let first_opt = line_split.next();

                match first_opt {
                    Some(value) => match value {
                        "GET" | "POST" => {
                            match value {
                                "GET" => request = Request::GET,
                                "POST" => request = Request::POST,
                                _ => {}
                            };
                            url = String::from(line_split.next().unwrap());
                        }
                        "Accept:" => {
                            let types_str = line_split.next().unwrap().split(",");

                            for typ in types_str {
                                match typ {
                                    "text/html" => accept_types.push(AcceptTypes::TextHtml),
                                    "image/webp" => accept_types.push(AcceptTypes::ImageWebp),
                                    "*/*" => accept_types.push(AcceptTypes::WildcardWildcard),
                                    _ => {}
                                };
                            }
                        }
                        "Connection:" => {
                            let connection_str = line_split.next().unwrap();
                            connection = match connection_str {
                                "close" => Connection::Close,
                                _ => Connection::KeepAlive,
                            };
                        }
                        "Content-Length:" => {
                            body = 1;
                            let len_str = line_split.next().unwrap();
                            let len = len_str.parse::<u32>().unwrap();
                            body_len = len;
                        }
                        "" if body == 1 => body = 2,

                        _ if body == 2 => {
                            let l = value.len() as u32;
                            let mut add: Vec<u8>;
                            if body_len < l {
                                add = value.as_bytes()[0..body_len as usize].to_vec();
                                body_len = 0;
                            } else {
                                add = value.as_bytes().to_vec();
                                body_len -= l;
                            }
                            body_value.append(&mut add);
                        }
                        _ => {}
                    },
                    None => println!("none"),
                };
            }

            "".len();

            if s < 1024 {
                break;
            }
        }

        Ok(Http {
            stream,

            request,
            url: url::Url::new(url),

            connection,

            status: 0,

            accept_types,

            body: body_value,
        })
    }

    pub fn send(&mut self, s: String) -> usize {
        self.stream.write(s.as_bytes()).unwrap()
    }

    pub fn send_buf(&mut self, buf: &[u8]) -> usize {
        self.stream.write(buf).unwrap()
    }

    pub fn send_vec(&mut self, buf: Vec<u8>) -> usize {
        self.stream.write(buf.as_ref()).unwrap()
    }

    pub fn write_response(&mut self, response: Vec<u8>) {
        let content_response = match self.accept_types.get(0).unwrap() {
            AcceptTypes::TextHtml => "text/html",
            AcceptTypes::ImageWebp => "image/webp",
            AcceptTypes::WildcardWildcard => "application/javascript",
        };

        let status_response = match self.status {
            200 => "200 OK",
            404 => "404 Not Found",
            _ => "404 Not Found",
        };

        let connection_response = match self.connection {
            Connection::Close => "close",
            Connection::KeepAlive => "keep-alive",
        };

        let header = format!(
            "HTTP/1.1 {}
Content-Length: {}
Content-Type: {}
Connection: {}\r\n\r\n",
            status_response,
            response.len(),
            content_response,
            connection_response
        );

        self.send(header);
        self.send_vec(response);
    }
}
