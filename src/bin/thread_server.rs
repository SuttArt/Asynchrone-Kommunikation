use std::{env, net::{TcpListener, TcpStream}, thread};
use std::io::{self, Write, BufReader, BufRead};
use std::net::SocketAddr;
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
				clients.lock().unwrap().push(stream.try_clone()?);

				thread::spawn(move || {
					handle_client(stream, clients);
				});
			}
			Err(e) => {
				eprintln!("Connection error: {:?}", e);
			}
		}
	}
	
	Ok(())
}


fn handle_client(stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
	let peer_addr = stream.peer_addr().unwrap();

	// Send to all, that client entered the chat room.
	let message = format!("The client {} entered the chat room\n", peer_addr);
	broadcast_clients(clients.clone(), peer_addr, message);

	println!("Connected with client {:?}", peer_addr);
	let reader = BufReader::new(&stream);

	for line in reader.lines() {
		match line {
			Ok(message) => {
				println!("Message from {}: {}", peer_addr, message.trim());

				// Send message to all connected clients
				let message = format!("{}: {}\n", peer_addr, message.trim());
				broadcast_clients(clients.clone(), peer_addr, message);
			},
			Err(e) => {
				eprintln!("Errors when reading {}: {}", peer_addr, e);
				break;
			}
		}
	}

	println!("Client {} has closed the connection.", peer_addr);

	// Send to all, that client left the chat room.
	let message = format!("The client {} left the chat room.\n", peer_addr);
	broadcast_clients(clients.clone(), peer_addr, message);

	// Remove the client from the list of connected clients when it has closed the connection
	let mut clients_mut = clients.lock().unwrap();
	clients_mut.retain(|client| client.peer_addr().unwrap() != peer_addr);
	println!("Client {} has been removed.", peer_addr);
}


fn broadcast_clients(clients: Arc<Mutex<Vec<TcpStream>>>, peer_addr: SocketAddr, message: String) {
	let clients_mut = clients.lock().unwrap();
	for mut client in clients_mut.iter() {
		if client.peer_addr().unwrap() != peer_addr {
			if let Err(e) = client.write_all(message.as_bytes()) {
				eprintln!("Error when sending to {}: {}", client.peer_addr().unwrap(), e);
			}
		}
	}
}