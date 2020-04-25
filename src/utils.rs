pub fn namespacified(input: &str) -> String {
	input.to_lowercase().replace(" ", "_")
}