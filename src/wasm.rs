use std::{io, mem, os::fd::AsRawFd};
use wasi::{wasi_snapshot_preview1::fd_filestat_get, Filestat};
use crate::{GetID, FileID};

pub type FileIDImpl = (u64, u64);

impl<T: AsRawFd> GetID for T {
	fn get_id(&self) -> io::Result<FileID> {
		let fd = self.as_raw_fd();
		unsafe {
			let mut filestat: Filestat = mem::zeroed();
			match fd_filestat_get(fd, &mut filestat as *mut _ as i32) {
				0 => Ok(FileID((filestat.dev, filestat.ino))),
				_ => todo!(),
			}
			/*let mut buf = mem::zeroed();
			if libc::fstat64(fd, &mut buf) == 0 {
				Ok(FileID((buf.st_dev, buf.st_ino)))
			} else {
				Err(io::Error::last_os_error())				
			}*/
		}
	}
}
