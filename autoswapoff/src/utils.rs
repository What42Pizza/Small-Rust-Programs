use crate::*;

pub fn fs_read_to_string(path: impl AsRef<Path>) -> Result<String> {
	let path = path.as_ref();
	std::fs::read_to_string(path).with_context(|| format!("Failed to read file {path:?}"))
}

pub fn take_arg<T>(args: &mut impl Iterator<Item = String>, arg_name: &str) -> Result<T>
where
	T: FromStr,
	<T as FromStr>::Err: std::error::Error + std::fmt::Display + Send + Sync + 'static,
{
	args.next()
		.ok_or_else(|| anyhow!("unexpected end of command arguments"))?
		.parse()
		.with_context(|| anyhow!("failed to parse '{}' argument", arg_name))
}
