use std::{io, mem, os::windows::io::AsRawHandle, ffi::c_void};
use winapi::um::{winbase::GetFileInformationByHandleEx, minwinbase::FileIdInfo, fileapi::FILE_ID_INFO};
use crate::{GetID, FileID};

pub type FileIDImpl = (u64, u128);

impl<T: AsRawHandle> GetID for T {
	fn get_id(&self) -> io::Result<FileID> {
		let handle = self.as_raw_handle();
		unsafe {
			let mut info: FILE_ID_INFO = mem::zeroed();
			if GetFileInformationByHandleEx(
				handle,
				FileIdInfo,
				&mut info as *mut _ as *mut c_void,
				mem::size_of_val(&info) as u32
			) == 0 {
				Err(io::Error::last_os_error())	
			} else {
				Ok(FileID((info.VolumeSerialNumber, u128::from_ne_bytes(info.FileId.Identifier))))				
			}
		}
	}
}
