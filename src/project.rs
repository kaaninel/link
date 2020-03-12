use std::collections::HashMap;
use serde::{Deserialize};
use std::fs;
use std::path::PathBuf;
use toml::{de::Error};
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
	workspace: Workspace,
	project: Option<HashMap<String, Project>>,
	interface: Option<HashMap<String, Interface>>,
	service: Option<HashMap<String, Service>>,	
}

#[derive(Deserialize, Debug)]
pub struct Workspace {
	projects: Vec<String>,
	services: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Project {
	interface: String,
}

#[derive(Deserialize, Debug)]
pub struct Interface {
}

#[derive(Deserialize, Debug)]
pub struct Service {
}

impl Config{
	pub fn load() -> Result<Config, Error> {
		let project_path = match env::var_os("project_path") {
			Some(x) => PathBuf::from(x).join("link.toml"),
			None => PathBuf::from("link.toml")
		}.canonicalize().unwrap(); 
		
		let contents = fs::read_to_string(project_path)
			.expect("Something went wrong reading the file");
		
		let result = toml::from_str(&contents)
			.expect("Couldn't parse link.toml file");

		Ok(result)
	}
}