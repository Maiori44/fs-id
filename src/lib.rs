use std::{fs::File, io, path::Path};

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(unix, path = "unix.rs")]
mod sys;

/// A file's identifier, can be compared with other `FileID`s to check if 2 variables point to the same file.
/// 
/// This struct is the combination of 2 identifiers:
/// 
/// * The id of the storage that contains the file.
/// * The internal file id, unique only across files in the same storage.
/// 
/// Combining both allows to uniquely identify the file within the entire system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileID (sys::FileIDImpl);

impl FileID {
	/// Given a path, obtain the identifier of a file, directory, etc.
	/// 
	/// # Platform-specific behavior
	/// 
	/// While on Unix obtaining the identifier of a directory is possible,
	/// on Windows an error will be returned instead.
	/// 
	/// This function uses `fstat64` on Unix and `GetFileInformationByHandleEx` on Windows.  
	/// This may change in the future.
	/// 
	/// # Errors
	///
	/// This function will error if it fails to open the file
	/// or fails to obtain the metadata containing the identifier.
	/// 
	/// # Examples
	/// 
	/// ```rust,no_run
	/// use fs_id::FileID;
	/// 
	/// fn main() -> std::io::Result<()> {
	///     let file_id1 = FileID::new("/some/file/path.txt")?;
	///     let file_id2 = FileID::new("/some/file/path.txt")?;
	///     let file_id3 = FileID::new("/some/other/file.txt")?;
	///     assert_eq!(file_id1, file_id2);
	///     assert_ne!(file_id1, file_id3);
	///     Ok(())
	/// }
	/// ```
	pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		File::open(path)?.get_id()
	}

	/// Returns the storage identifier from the file identifier.
	/// 
	/// # Platform-specific behavior
	/// 
	/// This returns `st_dev` on Unix and `VolumeSerialNumber` on Windows.  
	/// This may change in the future.
	/// 
	/// # Examples
	/// 
	/// ```rust,no_run
	/// use fs_id::FileID;
	/// 
	/// fn main() -> std::io::Result<()> {
	///     let file_id = FileID::new("/some/file/path.txt")?;
	///     println!("{}", file_id.storage_id());
	///     Ok(())
	/// }
	/// ```
	#[must_use]
	pub const fn storage_id(&self) -> u64 {
		self.0.0
	}

	/// Returns the internal file identifier from the file identifier.  
	/// Note that this value alone cannot uniquely identify the file within the system.
	/// 
	/// # Platform-specific behavior
	/// 
	/// This returns `st_ino` on Unix and `FileId` on Windows.  
	/// This may change in the future.
	/// 
	/// On Unix only 64 of the returned 128 bits are effectively used.
	/// 
	/// # Examples
	/// 
	/// ```rust,no_run
	/// use fs_id::FileID;
	/// 
	/// fn main() -> std::io::Result<()> {
	///     let file_id = FileID::new("/some/file/path.txt")?;
	///     println!("{}", file_id.internal_file_id());
	///     Ok(())
	/// }
	/// ```
	#[must_use]
	pub const fn internal_file_id(&self) -> u128 {
		self.0.1 as u128
	}
}

/// A trait to obtain the file identifier of an underlying object.
pub trait GetID {
	/// Obtains the file identifier, see [`FileID::new`] for more information.
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
		println!("id1: {id1:?}\nid2: {id2:?}\nid3: {id3:?}");
		Ok(())
	}
}
