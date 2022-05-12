use serde::Deserialize;
use std::fs::read_to_string;
use std::io::ErrorKind;

/*
This mod opens a config.json file, which contains the newsapi key and user preferences.
At the moment, there are no user preferences, only the API key.
*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
	// the api key needed for the newsapi
	api_key: String,
	source: String
}

// entry point to module
pub fn load_config() -> Config {

	let config_filepath = "config.json";
	
	let file_contents = open_file(config_filepath);

	let configuration = parse_json(file_contents);

	configuration
	
}

// reads the contents of the config file
fn open_file(filepath: &'static str) -> String {

	// read config file
	let file_contents = read_to_string(filepath);
	// match error handling, to try to differentiate errors rather than just panicking
	let file_contents = match file_contents {
		Ok(file_contents) => file_contents,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => panic!("Config file could not be found."),
			_other_error => panic!("Error trying to open config file.")
		}
	};

	file_contents
}

// the config file is in the JSON format, so it needs to be deserialized and stored in a config struct
fn parse_json(file_contents: String) -> Config {

	// deserialise config file into config struct
	let configuration: Config = serde_json::from_str(&file_contents).expect("JSON format incorrect; could not parse.");

	configuration
}




#[cfg(test)]
mod tests {
	
	// test a file string that is formatted correctly
	#[test]
	fn test_correct_deserialise() {
		let correct_json_string: String = "{ \"ApiKey\":\"123\", \"Source\":\"bbc-news\"}".to_string();
		let configuration: super::Config = super::parse_json(correct_json_string);
		assert_eq!("123", configuration.api_key);
	}

	// test a file string that is not in a JSON format
	#[test]
	#[should_panic]
	fn test_incorrect_json() {
		let incorrect_json_string: String = "ioaeenrst".to_string();
		let _configuration: super::Config = super::parse_json(incorrect_json_string);
	}

	// test a JSON file string that does not contain the correct parameters
	#[test]
	#[should_panic]
	fn test_incorrect_config() {
		let incorrect_config_string: String = "{ \"NotApiKey\":\"123\"}".to_string();
		let _configuration: super::Config = super::parse_json(incorrect_config_string);
	}
}