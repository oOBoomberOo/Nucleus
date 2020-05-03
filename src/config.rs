use std::collections::HashMap;

/// Hold config variable that will be used to format [Template](struct.Template.html)
/// ```
/// # use nucleus::Config;
/// let content = "Message: <foo>";
/// let config = Config::new()
///     .insert("<foo>", "hello, world!");
///
/// assert_eq!(config.apply(content), "Message: hello, world!");
/// ```
#[derive(Default)]
pub struct Config {
	value: HashMap<String, String>,
}

impl Config {
	pub fn new() -> Config {
		Config::default()
	}

	/// Insert a config variable
	pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Config {
		let key = key.into();
		let value = value.into();
		self.value.insert(key, value);
		self
	}

	/// Apply the current config to the given string
	pub fn apply(&self, content: impl Into<String>) -> String {
		let content = content.into();
		self.value
			.iter()
			.fold(content, |content, (key, value)| content.replace(key, value))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn replace_once() {
		let content = r#"Replace <foo> once"#;
		let config = Config::new().insert("<foo>", "me");

		assert_eq!(config.apply(content), "Replace me once");
	}

	#[test]
	fn replace_multiple() {
		let content = r#"
		Line 1: <foo>
		Line 2: <bar>
		Line 3: <foobar>
		"#;
		let config = Config::new()
			.insert("<foo>", "一")
			.insert("<bar>", "二")
			.insert("<foobar>", "三");

		let result = config.apply(content);
		let expect = r#"
		Line 1: 一
		Line 2: 二
		Line 3: 三
		"#;

		assert_eq!(result, expect);
	}

	#[test]
	fn replace_many_time() {
		let content = r#"
		A: <explosion>
		B: <explosion>
		C: <explosion>
		D: <explosion>
		"#;
		let config = Config::new().insert("<explosion>", "エクプロシオン");

		let result = config.apply(content);
		let expect = r#"
		A: エクプロシオン
		B: エクプロシオン
		C: エクプロシオン
		D: エクプロシオン
		"#;

		assert_eq!(result, expect);
	}
}
