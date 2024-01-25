use std::{fs::File, io, path::Path};

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(unix, path = "unix.rs")]
mod sys;

#[derive(Debug, PartialEq, Eq)]
pub struct FileID (sys::FileIDImpl);

impl FileID {
	pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		File::open(path)?.get_id()
	}
}

pub trait GetID {
	fn get_id(&self) -> io::Result<FileID>;
}

impl GetID for Path {
	fn get_id(&self) -> io::Result<FileID> {
		FileID::new(self)
	}
}

#[cfg(test)]
mod tests {
	use crate::FileID;

	#[test]
	fn check_comparisons() -> std::io::Result<()> {
		let id1 = FileID::new("Cargo.toml")?;
		let id2 = FileID::new("Cargo.toml")?;
		let id3 = FileID::new("LICENSE")?;
		assert_eq!(id1, id2);
		assert_ne!(id1, id3);
		Ok(())
	}
}
