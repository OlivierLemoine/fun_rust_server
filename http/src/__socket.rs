use std::io::prelude::{Read, Write};
use std::net::TcpStream;

pub struct Stream<'a> {
    pub message: String,
    pub stream: &'a TcpStream,
}

pub struct Socket<'a> {
    pub stream: &'a TcpStream,
}

impl<'a> Iterator for Socket<'a> {
    type Item = Stream<'a>;

    fn next(&mut self) -> Option<Stream<'a>> {
        // let mut buf = [0, 512];
        let mut buf = vec![0; 512];
        let s = self.stream.read(&mut buf[..]);
        if s.unwrap() == 0 {
            return None;
        }
        Some(Stream {
            message: String::from_utf8(buf).unwrap(),
            stream: &self.stream,
        })
    }
}