use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read};

#[derive(Debug, PartialEq, Eq)]
pub struct PanicPayload {
	pub msg: Option<String>,
	pub file: Option<String>,
	pub line: Option<u32>,
	pub col: Option<u32>,
}

fn read_string(rdr: &mut io::Cursor<&[u8]>) -> io::Result<Option<String>> {
	let string_len = rdr.read_u32::<LittleEndian>()?;
	let string = if string_len == 0 {
		None
	} else {
		let mut content = vec![0; string_len as usize];
		rdr.read_exact(&mut content)?;
		Some(String::from_utf8_lossy(&content).into_owned())
	};
	Ok(string)
}

pub fn decode(raw: &[u8]) -> PanicPayload {
	let mut rdr = io::Cursor::new(raw);
	let msg = read_string(&mut rdr).ok().and_then(|x| x);
	let file = read_string(&mut rdr).ok().and_then(|x| x);
	let line = rdr.read_u32::<LittleEndian>().ok();
	let col = rdr.read_u32::<LittleEndian>().ok();
	PanicPayload {
		msg: msg,
		file: file,
		line: line,
		col: col,
	}
}
