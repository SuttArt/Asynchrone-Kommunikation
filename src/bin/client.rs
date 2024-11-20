use std::{env, net::TcpStream, thread};
use std::io::{self, Write, BufReader, BufRead};

fn main() -> io::Result<()> {
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8000"));
	let mut _stream = TcpStream::connect(addr)?;

	// create clone stream, to be able to read and write at the same time
	let stream_clone = _stream.try_clone()?;

	thread::spawn(move || {
		let reader = BufReader::new(stream_clone);
		for line in reader.lines() {
			match line {
				Ok(message) => println!("Server: {}", message),
				Err(e) => {
					eprintln!("Error when reading the message: {}", e);
					break;
				}
			}
		}
	});

	// Haupt-Thread liest Nachrichten von der Konsole und sendet sie an den Server
	let mut input = String::new();
	loop {
		input.clear();
		io::stdin().read_line(&mut input)?;
		_stream.write_all(input.as_bytes())?;
	}
}
