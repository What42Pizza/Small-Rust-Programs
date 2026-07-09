use crate::*;



pub fn fs_read_to_string(path: impl AsRef<Path>) -> Result<String> {
	let path = path.as_ref();
	std::fs::read_to_string(path).with_context(|| format!("Failed to read file {path:?}"))
}
