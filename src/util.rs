#[macro_export]
macro_rules! context {
	($a:expr) => {
		|| concat!(" ", file!(), ":", line!(), " ").to_owned() + &$a
	};
}
