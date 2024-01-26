use std::{ffi::OsStr, fs::File, io, path::Path};

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
	/// Obtains the identifier of a file, directory, etc.
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
	/// Basic usage:
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
	/// 
	/// Many different types can be used:
	/// 
	/// ```rust,no_run
	/// use fs_id::FileID;
	/// 
	/// fn main() -> std::io::Result<()> {
	///     let file_id1 = FileID::new("using_str.txt")?;
	///     let file_id2 = FileID::new(std::ffi::OsStr::new("using_os_str.txt"))?;
	///     let file_id3 = FileID::new(&std::fs::File::open("using_a_file.txt")?)?;
	///     let file_id4 = FileID::new(&std::io::stdout())?;
	///     // etc...
	///     Ok(())
	/// }
	/// ```
	pub fn new<T: GetID + ?Sized>(file: &T) -> io::Result<Self> {
		file.get_id()
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

impl GetID for FileID {
	/// Returns a copy of itself wrapped inside `Ok`.
	fn get_id(&self) -> io::Result<FileID> {
		Ok(self.to_owned())
	}
}

macro_rules! impl_get_id {
	($($type:ty),+) => {
		$(
			impl GetID for $type {
				fn get_id(&self) -> io::Result<FileID> {
					File::open(self)?.get_id()
				}
			}
		)+
	};
}

impl_get_id!(Path, str, OsStr);

/// Compares 2 different file identifiers, and returns `Ok(true)` if the 2 identifiers point to the same file,
/// returning `Ok(false)` otherwise.
/// 
/// See [`FileID::new`] for more information on the identifiers.
/// 
/// # Errors 
/// 
/// Returns [`io::Error`] when failing to obtain any of the 2 identifiers.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use fs_id::compare_ids;
/// 
/// fn main() -> std::io::Result<()> {
///     println!("{}", compare_ids("/some/file/path.txt", "/some/other/path.txt")?);
///     // works with more than just file paths....
///     println!("{}", compare_ids(&std::io::stdout(), &std::io::stderr())?);
///     Ok(())
/// }
/// ```
pub fn compare_ids<T1: GetID + ?Sized, T2: GetID + ?Sized>(id1: &T1, id2: &T2) -> io::Result<bool> {
	Ok(id1.get_id()? == id2.get_id()?)
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
