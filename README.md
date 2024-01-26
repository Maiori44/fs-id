# fs-id

Little (mostly) cross-platform library to uniquely identify files (and other things).

For quick comparisons, `compare_ids` can be used:

```rust
use fs_id::compare_ids;

fn main() -> std::io::Result<()> {
	assert!(compare_ids("/some/file/path.txt", "/some/file/path.txt")?);
	Ok(())
}
```

For more advanced usage, `FileID` can be used:

```rust
use fs_id::FileID;

fn main() -> std::io::Result<()> {
	let file_id1 = FileID::new("/some/file/path.txt")?;
	let file_id2 = FileID::new("/some/file/path.txt")?;
	let file_id3 = FileID::new("/some/other/file.txt")?;
	assert_eq!(file_id1, file_id2);
	assert_ne!(file_id1, file_id3);
	println!("{} {}", file_id1.storage_id(), file_id1.internal_file_id());
	Ok(())
}
```
