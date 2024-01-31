use std::io;
#[macro_export]
macro_rules! context {
	($a:expr) => {
		|| concat!(" ", file!(), ":", line!(), " ").to_owned() + &$a
	};
}

pub enum CrlfWriter<W: io::Write> {
	Enabled(W),
	Disabled(W),
}

impl<W: io::Write> io::Write for CrlfWriter<W> {
	// Replace any \n to \r\n
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		match self {
			CrlfWriter::Enabled(inner) => {
				let mut count = 0;

				for &byte in buf.iter() {
					if byte == b'\n' {
						// Write \r before \n
						count += 1;
						inner.write_all(b"\r\n")?;
					} else {
						count += inner.write(&[byte])?;
					}
				}

				Ok(count)
			},
			CrlfWriter::Disabled(inner) => inner.write(buf),
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		match self {
			CrlfWriter::Enabled(i) | CrlfWriter::Disabled(i) => i.flush(),
		}
	}
}
