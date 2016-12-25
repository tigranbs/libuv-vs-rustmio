extern crate mio;
extern crate slab;

use mio::{Token, Poll, Ready, PollOpt, Events};
use mio::tcp::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::str::FromStr;
use std::collections::VecDeque;
use std::io::{Read, Write, ErrorKind};

type Slab<T> = slab::Slab<T, Token>;

const SERVER_TOKEN: Token = Token(10_000_000);

struct Conn {
    socket: TcpStream,
    write_queue: VecDeque<Vec<u8>>
}

fn main() {
    // buffer for reading bath of data for better performance
    let mut readable_buffer = [0; 64000];

    // a list of connections _accepted_ by our server
    let mut conns = Slab::with_capacity(128);

    // making Poll object
    let poll = Poll::new().unwrap();

    // binding TCP server on port 8888
    let addr = SocketAddr::from_str("0.0.0.0:8888").unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // registering listener to poll service
    poll.register(&listener, SERVER_TOKEN, Ready::readable(), PollOpt::edge()).unwrap();

    // Create storage for events
    let mut events = Events::with_capacity(50000);

    // starting event loop
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            let (token, kind) = (event.token(), event.kind());

            // checking if we got error event or not
            if kind == Ready::error() || kind == Ready::hup() {
                // if we have an error on server socket,
                // then just exiting application
                if token == SERVER_TOKEN {
                    println!("Got Error on Server Listener, Exiting application !");
                    return;
                }

                // if error is for one of our client connections
                // just removing it from the slab, so it would be deallocated and would be closed
                conns.remove(token);
                continue;
            }

            // if we have socket ready to be read some data
            // or server socket is ready to accept connection
            if kind == Ready::readable() {
                match token {
                    SERVER_TOKEN => {
                        loop {
                            // accepting connection
                            let sock = match listener.accept() {
                                Ok((s, _)) => s,
                                Err(_) => break
                            };

                            if conns.vacant_entry().is_none() {
                                let amt = conns.len();
                                conns.reserve_exact(amt);
                            }
                            let entry = conns.vacant_entry().unwrap();
                            let c = Conn{
                                socket: sock,
                                write_queue: VecDeque::new()
                            };

                            poll.register(&c.socket, entry.index(), Ready::readable(), PollOpt::edge()).unwrap();


                            entry.insert(c);
                        }
                    }

                    _ => {
                        let mut need_to_close = false;
                        {
                            // trying to get Client connection based on Token
                            // if we don't have connection with that token, then just moving forward to the next event
                            let ref mut conn = conns[token];

                            // reading socket data until the end
                            loop {
                                // trying to read data from socket
                                match conn.socket.read(&mut readable_buffer) {
                                    Ok(rs) => {
                                        // if we don't have error and data is not available
                                        // then probably it is EOF for this socket, so we need to close connection
                                        if rs == 0 {
                                            need_to_close = true;
                                            break;
                                        } else {
                                            // writing buffer to socket and getting size of how match bytes written
                                            // if we are getting an error then we will try to write to connecton on next cycle
                                            let write_size = match conn.socket.write(&readable_buffer[0..rs]) {
                                                Ok(ws) => ws,
                                                Err(_) => {
                                                    // we need to add buffer back to write it later
                                                    conn.write_queue.push_back(Vec::from(&readable_buffer[0..rs]));

                                                    // making connection writable to fire write event
                                                    poll.reregister(&conn.socket, token, Ready::readable() | Ready::writable(), PollOpt::edge()).unwrap();
                                                    break;
                                                }
                                            };

                                            // if we have pending data for this buffer
                                            // then we need to split buffer and make connection writable again
                                            if write_size < rs {
                                                // adding pending data back to the queue for writing it again
                                                conn.write_queue.push_back(Vec::from(&readable_buffer[write_size..rs]));

                                                // making connection writable to fire write event
                                                poll.reregister(&conn.socket, token, Ready::readable() | Ready::writable(), PollOpt::edge()).unwrap();
                                                break;
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        // if woud connection is giving WouldBlock error
                                        // then it's can't give data at this time, but connection don't have an error
                                        if e.kind() != ErrorKind::WouldBlock {
                                            need_to_close = true;
                                        }

                                        break;
                                    }
                                };
                            }
                        }

                        // if we need to close connection then just removing it from our list
                        if need_to_close {
                            conns.remove(token);
                        }
                    }
                }

                continue;
            }

            if kind == Ready::writable() {
                match token {
                    SERVER_TOKEN => {
                        println!("It is impossible but we got Writable event for server socket :) Exiting !!");
                        return;
                    }

                    _ => {
                        // trying to get Client connection based on Token
                        // if we don't have connection with that token, then just moving forward to the next event
                        let ref mut conn = conns[token];

                        // trying to write all writable queue
                        loop {
                            // poping first element from queue
                            // or ending queue loop if we don't have 0 elements
                            let buf = match conn.write_queue.pop_front() {
                                Some(b) => b,
                                None => break
                            };

                            // writing buffer to socket and getting size of how match bytes written
                            // if we are getting an error then we will try to write to connecton on next cycle
                            let write_size = match conn.socket.write(buf.as_slice()) {
                                Ok(ws) => ws,
                                Err(_) => {
                                    // we need to add buffer back to write it later
                                    conn.write_queue.push_front(buf);
                                    break;
                                }
                            };

                            // if we have pending data for this buffer
                            // then we need to split buffer and make connection writable again
                            if write_size < buf.len() {
                                // adding pending data back to the queue for writing it again
                                conn.write_queue.push_front(Vec::from(&buf[write_size..]));
                                break;
                            }
                        }

                        // if we don't have pending data to be written
                        // then we can make connection only readable again
                        if conn.write_queue.len() == 0 {
                            poll.reregister(&conn.socket, token, Ready::readable(), PollOpt::edge()).unwrap();
                        }
                    }
                }
            }
        }
    }
}
