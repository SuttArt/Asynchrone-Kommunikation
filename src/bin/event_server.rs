#![allow(unused_imports)]
use futures::AsyncWriteExt;
use mio::{net::TcpListener, Events, Interest, Poll, Token};
use std::iter::zip;
use std::{
    env,
    io::{self, ErrorKind, Read, Write},
};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use mio::net::TcpStream;

fn main() -> io::Result<()> {
    let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8000"));
    println!("start server {}", addr);
    let mut listener = TcpListener::bind(addr.parse().unwrap())?;
    println!("server started ({})", listener.local_addr().unwrap());
    let mut events = Events::with_capacity(4);
    let mut poll = Poll::new()?;
    let mut clients = vec![];
    let mut buf = [0u8; 256];

    poll.registry()
        .register(&mut listener, Token(0), Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                Token(0) => {
                    while let Ok((mut stream, _sender)) = listener.accept() {
                        print!("client {} connected as ", stream.peer_addr().unwrap());
                        poll.registry().register(
                            &mut stream,
                            Token(clients.len().wrapping_add(1)),
                            Interest::READABLE,
                        )?;
                        clients.push(Some(stream));
                        println!("{}", clients.len());
                    }
                }

                // event based solution (single threaded)
                Token(n) => unsafe {
                    let clients = &mut clients as *mut Vec<Option<TcpStream>>;
                    let stream = (&mut (*clients)[n - 1]).as_mut().unwrap();
                    let disconnect = 'outer: loop {
                        match stream.read(&mut buf) {
                            Ok(bytes) => {
                                if bytes == 0 {
                                    break 'outer true;
                                }

                                for (i, push_stream) in zip(1..(*clients).len() + 1, (*clients).as_slice().iter()) {
                                    if push_stream.is_none() {                  // if no one is connected
                                        continue;
                                    }
                                    println!("n={}, i={}, {}", n, i, push_stream.as_ref().unwrap().peer_addr().unwrap());
                                    if i != n {                                 // no self sending
                                        println!("Sending {:?} to {:?}",
                                                 std::str::from_utf8(&buf[0..bytes]).unwrap(),
                                                 push_stream.as_ref().unwrap().peer_addr().unwrap());
                                        let escape = push_stream.as_ref().unwrap().write(&buf[0..bytes]);
                                        println!("sent {} bytes to {}", escape.unwrap(), push_stream.as_ref().unwrap().peer_addr().unwrap());
                                    } else {
                                        println!("own address (not sending)");
                                    }
                                }
                            }

                            Err(e) => {
                                break e.kind() != ErrorKind::WouldBlock;
                            }
                        }
                    };

                    if disconnect {
                        poll.registry().deregister(stream)?;
                        print!("client {} disconnected", n);
                        (*clients)[n - 1] = None;               // deregister client
                        println!(", {} clients left", (*clients).len());
                    }
                }
            }
        }
    }
}
