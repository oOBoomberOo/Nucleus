mod validator;
mod template;
mod config;
mod utils;

use std::path::PathBuf;
use anyhow::Result;
use dialoguer::{theme::{ColorfulTheme, Theme}, Input};
use dialoguer::Validator;
use structopt::StructOpt;
use validator::*;
use config::Config;
use utils::*;
use template::Template;

fn main() {
	if let Err(e) = run() {
		eprintln!("{}", e);
	}
}

fn run() -> Result<()> {
	let command = Command::from_args();
	let theme = &ColorfulTheme::default();

	let datapack_name = datapack_name(&command, theme)?;
	let description = description(&command, theme)?;
	let player_name = player_name(&command, theme)?;
	let display_item = display_item(&command, theme)?;	
	
	let datapack = namespacified(&datapack_name);
	let namespace = namespacified(&player_name);

	let config = Config::new()
		.insert("<datapack_name>", &datapack_name)
		.insert("<datapack>", datapack)
		.insert("<description>", description)
		.insert("<namespace>", namespace)
		.insert("<player_name>", player_name)
		.insert("<display_item>", display_item);

	let templates = get_template(&config)?;

	let path = match command.state {
		State::New => PathBuf::from(datapack_name),
		State::Init => PathBuf::from("./")
	};

	templates.iter().try_for_each(|template| template.generate(&path))?;

	Ok(())
}

fn get_template(config: &Config) -> Result<Vec<Template>> {
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

fn datapack_name(option: &Command, theme: &dyn Theme) -> Result<String> {
	if let Some(datapack_name) = &option.datapack_name {
		DatapackNameValidator.validate(datapack_name)?;
		return Ok(datapack_name.to_owned());
	}

	let result = Input::<String>::with_theme(theme)
		.with_prompt("Datapack name")
		.allow_empty(false)
		.validate_with(DatapackNameValidator)
		.interact()?;

	Ok(result)
}

fn description(option: &Command, theme: &dyn Theme) -> Result<String> {
	if let Some(description) = &option.description {
		return Ok(description.to_owned());
	}

	let result = Input::<String>::with_theme(theme)
		.with_prompt("Description")
		.default("My amazing datapack".to_owned())
		.interact()?;
	Ok(result)
}

fn player_name(option: &Command, theme: &dyn Theme) -> Result<String> {
	if let Some(player_name) = &option.player_name {
		NameValidator.validate(player_name)?;
		return Ok(player_name.to_owned());
	}

	let result = Input::<String>::with_theme(theme)
	.with_prompt("Player name")
	.allow_empty(false)
	.validate_with(NameValidator)
	.interact()?;

	Ok(result)
}

fn display_item(option: &Command, theme: &dyn Theme) -> Result<String> {
	if let Some(display_item) = &option.display_item {
		NamespaceValidator.validate(display_item)?;
		return Ok(display_item.to_owned());
	}
	
	let result = Input::<String>::with_theme(theme)
		.with_prompt("Display Item ID")
		.allow_empty(false)
		.validate_with(NamespaceValidator)
		.interact()?;
	Ok(result)
}

#[derive(StructOpt, Debug)]
struct Command {
	/// Datapack's Display Name
	#[structopt(short = "d", long = "name")]
	datapack_name: Option<String>,

	/// Datapack's description
	#[structopt(short = "D", long = "desc")]
	description: Option<String>,

	/// Player's name
	#[structopt(short = "p", long = "player")]
	player_name: Option<String>,

	/// Item use inside advancement to display datapack
	#[structopt(short = "i", long = "item")]
	display_item: Option<String>,

	#[structopt(subcommand)]
	state: State
}

#[derive(StructOpt, Debug)]
enum State {
	/// Create new datapack project inside current directory
	New,
	/// Initialize datapack project from current directory
	Init
}