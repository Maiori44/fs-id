use std::{fs::File, io, path::Path};

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(unix, path = "unix.rs")]
mod sys;

pub trait GetID {
	fn get_id(&self) -> io::Result<FileID>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileID (sys::FileIDImpl);

impl FileID {
	pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		File::open(path)?.get_id()
	}
}
