use std::{io::{self, Read, Write}, env, net::{TcpListener, TcpStream}};

fn main() -> io::Result<()> {
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8000"));
	let listener = TcpListener::bind(addr)?;
	
	// TODO: implement threads for the server logic
	println!("Bound to {:?}", listener);
	for stream in listener.incoming() {
		// println!("Connection attempt by {:?}", stream?);

		let _ = handle_client(stream?);
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