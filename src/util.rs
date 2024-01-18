use std::io;
#[macro_export]
macro_rules! context {
	($a:expr) => {
		|| concat!(" ", file!(), ":", line!(), " ").to_owned() + &$a
	};
}


pub struct CrlfWriter<W: io::Write> {
	inner: W,
}

impl<W: io::Write> io::Write for CrlfWriter<W> {
	// Replace any \n to \r\n
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let mut count = 0;

		for &byte in buf.iter() {
			if byte == b'\n' {
				// Write \r before \n
				self.inner.write_all(&[b'\r'])?;
				count += 1;
			}
			self.inner.write_all(&[byte])?;
			count += 1;
		}

		Ok(count)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.inner.flush()
	}
}