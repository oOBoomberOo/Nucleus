use crate::config::Config;
use anyhow::Result;
use std::{
	fmt, fs,
	path::{Path, PathBuf},
};
use thiserror::Error;

pub struct Template {
	path: PathBuf,
	content: String,
}

impl Template {
	pub fn new(path: impl Into<PathBuf>, content: impl Into<String>) -> Template {
		let path = path.into();
		let content = content.into();
		Template { path, content }
	}

	pub fn from_str(content: &str, config: &Config) -> Result<Template> {
		let content = config.apply(content);
		let mut lines = content.lines().skip_while(|line| line.is_empty());
		let path: &str = lines
			.next()
			.map(|x| x.trim())
			.ok_or(TemplateError::UnexpectedEOF)?;
		
		if !path.starts_with('#') {
			return Err(TemplateError::PathSymbolNotFound.into());
		}

		let path: &str = path
			.get(1..)
			.map(|x| x.trim())
			.ok_or(TemplateError::UnexpectedEOF)?;
			
		let content: String = lines.map(|x| format!("{}\n", x)).collect();
		let result = Template::new(path, content);
		Ok(result)
	}

	fn clean_content(&self) -> String {
		self.content.split_whitespace().collect()
	}

	fn ensure_parent(&self, path: impl AsRef<Path>) -> Result<()> {
		if let Some(parent) = path.as_ref().parent() {
			fs::create_dir_all(parent)?;
		}

		Ok(())
	}

	pub fn generate(&self, root: impl AsRef<Path>) -> Result<()> {
		let root = root.as_ref();
		let path = root.join(&self.path);
		self.ensure_parent(&path)?;
		fs::write(path, &self.content)?;
		Ok(())
	}
}

impl PartialEq for Template {
	fn eq(&self, other: &Template) -> bool {
		self.path.eq(&other.path) && self.clean_content().eq(&other.clean_content())
	}
}

impl fmt::Debug for Template {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Template {{ path: {:?}, content: {:?} }}", self.path, self.clean_content())
	}
}

#[derive(Debug, Error, PartialEq)]
enum TemplateError {
	#[error("Unexpected End-of-File")]
	UnexpectedEOF,
	#[error("Path symbol not found")]
	PathSymbolNotFound
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_template() {
		let content = r#"
		# pack.mcmeta
			{
				"pack": {
					"pack_format": 5,
					"description": "This is a description"
				}
			}
		"#;

		let config = Config::new();

		let result = Template::from_str(content, &config).unwrap();
		let expect = Template::new(
			"pack.mcmeta",
			r#"
			{
				"pack": {
					"pack_format": 5,
					"description": "This is a description"
				}
			}
		"#,
		);

		assert_eq!(result, expect);
	}

	#[test]
	fn template_with_config() {
		let content = r#"
		# data/<namespace>/advancements/<datapack>.json
		{
			"display": {
				"title": "<datapack>",
				"description": "<description>",
				"icon": {
					"item": "<display_item>"
				},
				"announce_to_chat": false,
				"show_toast": false
			},
			"parent": "global:<namespace>",
			"criteria": {
				"trigger": {
					"trigger": "minecraft:tick"
				}
			}
		}		
		"#;

		let config = Config::new()
			.insert("<namespace>", "boomber")
			.insert("<datapack>", "explosion_magic")
			.insert("<description>", "Explosion!")
			.insert("<display_item>", "minecraft:tnt");

		let result = Template::from_str(content, &config).unwrap();
		let expect = Template::new(
			"data/boomber/advancements/explosion_magic.json",
			r#"
		{
			"display": {
				"title": "explosion_magic",
				"description": "Explosion!",
				"icon": {
					"item": "minecraft:tnt"
				},
				"announce_to_chat": false,
				"show_toast": false
			},
			"parent": "global:boomber",
			"criteria": {
				"trigger": {
					"trigger": "minecraft:tick"
				}
			}
		}
		"#,
		);

		assert_eq!(result, expect);
	}

	#[test]
	fn with_invalid_syntax() {
		let content = r#"Hello, world!"#;
		let config = Config::new();
		let result: TemplateError = Template::from_str(content, &config).unwrap_err().downcast().unwrap();
		assert_eq!(result, TemplateError::PathSymbolNotFound);
	}
}
