use serde::Deserialize;
use std::fs::File;
use std::io::ErrorKind;

/*
This mod opens a config.json file, which contains the newsapi key and user preferences.
At the moment, there are no user preferences, only the API key.
*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct config {
	api_key: String
}


pub fn load_config() -> config {

	// read config file
	let file = File::open("config.json");
	let file = match file {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => panic!("Config file could not be found."),
			other_error => panic!("Error trying to open config file.")
		}
	};
}