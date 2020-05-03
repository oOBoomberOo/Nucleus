use anyhow::Result;
use crate::{config::Config, template::Template};

/// Attempt to convert `input` into namespace.
/// * This function doesn't handle invalid namespace characters.
pub fn namespacified(input: &str) -> String {
	input.to_lowercase().replace(" ", "_")
}

/// Get template files with the given [Config](../struct.Config.html) applied
pub fn get_template_with_config(config: &Config) -> Result<Vec<Template>> {
	let templates = vec![
		include_str!("../template/datapack.template"),
		include_str!("../template/namespace.template"),
		include_str!("../template/pack.template"),
		include_str!("../template/root.template"),
	];
	templates.iter()
		.map(|content| Template::from_str(content, config))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn convert_to_namespace() {
		assert_eq!(
			namespacified("Boomber:Something Here"),
			"boomber:something_here"
		);

		assert_eq!(
			namespacified("Hello@World"),
			"hello@world"
		);

		assert_eq!(
			namespacified("test ()"),
			"test_()"
		);
	}
}