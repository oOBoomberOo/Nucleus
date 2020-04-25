use std::collections::HashMap;

pub struct Config {
	value: HashMap<String, String>,
}

impl Config {
	pub fn new() -> Config {
		let value = HashMap::new();
		Config { value }
	}

	pub fn insert(mut self, key: impl Into<String>, value: impl Into<String>) -> Config {
		let key = key.into();
		let value = value.into();
		self.value.insert(key, value);
		self
	}

	pub fn apply(&self, content: impl Into<String>) -> String {
		let content = content.into();
		self.value
			.iter()
			.fold(content, |content, (key, value)| content.replace(key, value))
	}
}