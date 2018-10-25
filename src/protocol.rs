use std::io;
// use std::result::Result;
// use std::error::Error;
use std::io::{Read, Write, BufRead, BufReader, Cursor};
use std::net::{TcpStream};
use std::thread;

extern crate byteorder;
use self::byteorder::{LittleEndian, ReadBytesExt};


const SFN_FILE: u8 = 0x01;
const SFN_DONE: u8 = 0x02;
const SFN_FILE_WITH_MD5: u8 = 0x03;

struct SMFileHeader {
	filename: String,
	size: u64,
}


fn send_files(mut stream: impl Write) -> io::Result<()> {
	println!("Local done.");
	stream.write(&[ SFN_DONE ])?;
	Ok(())
}

fn recv_files(mut stream: impl Read) -> io::Result<()> {
	fn recv_file(mut stream: impl BufRead) -> io::Result<()> {
		fn get_header(mut stream: impl BufRead) -> io::Result<SMFileHeader> {
			let mut filename: String = String::from("");
			stream.read_line(&mut filename)?;
			println!("  {}", filename);

			let size = stream.read_u64::<LittleEndian>()?;
			println!("  {} byte(s)", size);

			return Ok(SMFileHeader{ filename, size });
		}

		println!("Receiving a file");

		let header = get_header(stream);

		Ok(())
	}

	let mut stream = BufReader::new(stream);
	loop {
		let opcode = stream.read_u8()?;
		match opcode {
			SFN_DONE => {
				println!("Remote done.");
				return Ok(());
			},
			SFN_FILE => {
				recv_file(&mut stream)?;
				()
			},
			SFN_FILE_WITH_MD5 => panic!("Unsupported SM opcode: {}", opcode),
			_ => panic!("Unsupported SM opcode: {}", opcode),
		}
	}
}

pub fn handle_client(stream: TcpStream) -> io::Result<()> {
	let stream_clone = stream.try_clone()?;
	let send_thread = thread::spawn(move || {
		send_files(&stream_clone).unwrap();
	});
	let recv_thread = thread::spawn(move || {
		recv_files(&stream).unwrap();
	});

	send_thread.join().unwrap();
	recv_thread.join().unwrap();
	println!("All done, closing connection.");
	Ok(())
}
