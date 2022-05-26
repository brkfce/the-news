use serde::Deserialize;
use std::fs::read_to_string;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;

/*
This mod opens a config.json file, which contains the newsapi key and user preferences.
At the moment, there are no user preferences, only the API key.
*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
	// the api key needed for the newsapi
	pub api_key: String,
	// the source used to pull the headlines from
	#[serde(default = "source_default")]
	pub source: String,
	// the number of headlines to find and display
	#[serde(default = "number_of_headlines_default")]
	pub number_of_headlines: i32,
	// whether the headline or headline and description is displayed
	#[serde(default = "display_format_default")]
	pub display_format: String
}

// default values for config
fn source_default() -> String {
	"bbc-news".to_string()
}
fn number_of_headlines_default() -> i32 {
	10
}
fn display_format_default() -> String {
	"h&d&u".to_string()
}

// entry point to module
pub fn load_config() -> Result<Config, &'static str> {

	let config_filepath = "config.json";
	
	let file_contents = open_file(config_filepath)?;

	let configuration = parse_json(file_contents);

	configuration
	
}

// reads the contents of the config file
fn open_file(filepath: &'static str) -> Result<String, &'static str> {

	// read config file
	let file_contents = read_to_string(filepath);

	match file_contents {
		Ok(file_contents) => Ok(file_contents),
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match gen_file() {
				Ok(r) => Ok(r),
				Err(error) => Err(error), 
			}, 
			_other_error => Err("Error trying to open config file."),
		},
	}
}

// generate blank config file NOTE only ever returns an error, need to find a better way to return just an error
fn gen_file() -> Result<String, &'static str> {

	// create file
	let file = File::create("config.json");

	let mut file = match file {
		Ok(file) => file,
		Err(_error) => return Err("Could not find config file and failed to create one."),
	};

	// file file with empty config
	let write = file.write_all(b"{\n	\"ApiKey\":\"\",\n	\"Source\":\"bbc-news\",\n	\"NumberOfHeadlines\":10,\n	\"DisplayFormat\":\"h&d&u\"\n}");
	// returns appropriate error
	match write {
		Ok(_r) => return Err("No config file found, so a default config file has been created. Please populate it with your API key and try again."),
		Err(_e) => return Err("Default config file has been created, but template could not be written to it. Please fill in this file following the template in the README."),
	}
}

// the config file is in the JSON format, so it needs to be deserialized and stored in a config struct
fn parse_json(file_contents: String) -> Result<Config, &'static str> {

	// deserialise config file into config struct
	let configuration: Result<Config, serde_json::Error> = serde_json::from_str(&file_contents);

	let configuration = match configuration {
		Ok(configuration) => configuration,
		Err(_error) => return Err("JSON format incorrect, could not parse."),
	};

	if configuration.api_key == "" {
		panic!("API key not found. Please put an API key in the config.json file and try again.");
	}

	Ok(configuration)
}




#[cfg(test)]
mod tests {
	
	// test a file string that is formatted correctly
	#[test]
	fn test_correct_deserialise() {
		let correct_json_string: String = "{ \"ApiKey\":\"123\", \"Source\":\"bbc-news\"}".to_string();
		let configuration: super::Config = super::parse_json(correct_json_string).unwrap();
		assert_eq!("123", configuration.api_key);
	}

	// test a file string that is not in a JSON format
	#[test]
	#[should_panic]
	fn test_incorrect_json() {
		let incorrect_json_string: String = "ioaeenrst".to_string();
		let _configuration: super::Config = super::parse_json(incorrect_json_string).unwrap();
	}

	// test a JSON file string that does not contain the correct parameters
	#[test]
	#[should_panic]
	fn test_incorrect_config() {
		let incorrect_config_string: String = "{ \"NotApiKey\":\"123\"}".to_string();
		let _configuration: super::Config = super::parse_json(incorrect_config_string).unwrap();
	}
}
