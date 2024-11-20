use std::{env, net::TcpStream, thread};
use std::io::{self, Write, BufReader, BufRead};
use std::net::Shutdown;

fn main() -> io::Result<()> {
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8000"));
	let mut _stream = TcpStream::connect(addr)?;

	// green text: "\x1b[32m{}\x1b[0m"
	// Red text: "\x1b[31m{}\x1b[0m"
	println!("\x1b[32m{}\x1b[0m", "Welcome to the chat room, you have successfully joined.");
	println!("Write something in the CLI and send it by pressing the enter key");
	println!("{}\x1b[31m{}\x1b[0m{}", "Type \"", "exit", "\" if you want to leave the chat room.");

	// create clone stream, to be able to read and write at the same time
	let stream_clone = _stream.try_clone()?;

	thread::spawn(move || {
		let reader = BufReader::new(stream_clone);
		for line in reader.lines() {
			match line {
				Ok(message) => println!("{}", message),
				Err(e) => {
					if e.kind() == io::ErrorKind::ConnectionAborted
					{
						println!("Exiting...");
					} else {
						eprintln!("Error when reading the message: {}", e);
					}

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
		if input.trim().eq_ignore_ascii_case("exit") {
			println!("Connection terminated.");

			_stream.shutdown(Shutdown::Both)?;
			break;
		}
		_stream.write_all(input.as_bytes())?;
	}

	Ok(())
}
