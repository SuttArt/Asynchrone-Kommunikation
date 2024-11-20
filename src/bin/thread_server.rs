use std::{io::{self, Read, Write}, env, net::{TcpListener, TcpStream}, thread};
use std::sync::{Arc, Mutex};

fn main() -> io::Result<()> {
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8000"));
	let listener = TcpListener::bind(addr)?;
	
	println!("Bound to {:?}", listener);

	let clients = Arc::new(Mutex::new(Vec::new()));
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				let clients = Arc::clone(&clients);
				clients.lock().unwrap().push(stream.try_clone().unwrap());

				thread::spawn(move || {
					let _ = handle_client(stream);
				});
			}
			Err(e) => {
				eprintln!("Connection error: {:?}", e);
			}
		}
	}
	
	Ok(())
}


fn handle_client(mut stream: TcpStream) -> io::Result<()> {
	println!("Connection attempt by {:?}", stream.peer_addr()?);

	// Nachricht vom Client empfangen
	let mut buffer = [0; 128];
	let n = stream.read(&mut buffer)?;
	println!("Nachricht vom Client empfangen: {}", String::from_utf8_lossy(&buffer[..n]));

	// Eine Antwort an den Client senden
	stream.write(b"Hello, Client!")?;

	Ok(())
}