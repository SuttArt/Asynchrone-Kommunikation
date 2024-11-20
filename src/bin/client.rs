use std::{io, env, net::TcpStream};
use std::io::Write;

fn main() -> io::Result<()> {
	let addr = env::args().nth(1).unwrap_or(String::from("127.0.0.1:8000"));
	let mut _stream = TcpStream::connect(addr)?;

	_stream.write(&[1])?;
	// TODO: implement client logic
	
	Ok(())
}
