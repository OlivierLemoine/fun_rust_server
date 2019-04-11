use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

pub mod handler;
pub mod url;

pub struct Server {
    listener: TcpListener,
    waiting_sockets: Vec<TcpStream>,
    // waiting_sockets: Vec<TcpStream>,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        let listener = TcpListener::bind(addr).unwrap();

        let _ = listener.set_nonblocking(true);

        Server {
            listener,
            waiting_sockets: Vec::new(),
        }
    }

    pub fn listen(&mut self, f: impl Fn(&mut handler::Http)) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(s) => {
                    s.set_read_timeout(Some(Duration::new(0, 1))).unwrap();
                    self.waiting_sockets.insert(0, s);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if self.waiting_sockets.len() > 0 {
                        let sock = &self.waiting_sockets[0];

                        let mut buf = [0; 1];
                        if match sock.peek(&mut buf) {
                            Err(_e) => true,
                            Ok(0) => false,
                            Ok(_v) => {
                                let mut handler = match handler::Http::parse(&mut self.waiting_sockets[0]) {
                                    Ok(h) => h,
                                    Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => {
                                        return;
                                    }
                                    Err(e) => {
                                        panic!("{}", e);
                                    }
                                };
                                f(&mut handler);
                                match handler.connection {
                                    handler::Connection::Close => false,
                                    handler::Connection::KeepAlive => true,
                                }
                            }
                        } {
                            if self.waiting_sockets.len() > 10 {
                                self.waiting_sockets.pop();
                            } else {
                                self.waiting_sockets.rotate_right(1);
                            }
                        } else {
                            self.waiting_sockets.pop();
                        }
                    } else {
                        sleep(Duration::from_micros(50));
                    }
                }
                Err(e) => panic!("Io err : {}", e),
            }
        }
    }
}
