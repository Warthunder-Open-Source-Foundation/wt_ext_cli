use std::io;
#[macro_export]
macro_rules! context {
	($a:expr) => {
		|| concat!(" ", file!(), ":", line!(), " ").to_owned() + &$a
	};
}

#[macro_export]
macro_rules! arced {
	( $( $a:ident ),* ) => {
		$(
			crate::clone_arc!($a);
		)*
	};
}

#[macro_export]
macro_rules! clone_arc {
	($a:ident) => {
		let $a = $a.clone();
	};
}

pub enum CrlfWriter<W: io::Write> {
	// Writes every newline as carriage-return newline
	Enabled(W),
	// Passes through every call to W
	Disabled(W),
	// Does nothing
	Null,
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
			CrlfWriter::Null => Ok(buf.len()),
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		match self {
			CrlfWriter::Enabled(i) | CrlfWriter::Disabled(i) => i.flush(),
			CrlfWriter::Null => Ok(()),
		}
	}
}
