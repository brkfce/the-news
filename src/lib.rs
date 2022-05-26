mod fetch_data;
mod config;
mod display;


pub fn get_the_news() {

	// read in user config
	let user_config: Result<config::Config, &'static str> = config::load_config();
	let user_config = match user_config {
		Ok(user_config) => user_config,
		Err(error) => {
			println!("Error reading configuration file: {error:?}");
			std::process::exit(1)
		}
	};


	let response: Result<fetch_data::ResponseObject, &'static str> = fetch_data::fetch_data(&user_config);
	let response = match response {
		Ok(response) => response,
		Err(error) => {
			println!("Error getting response from API: {error:?}");
			std::process::exit(1)
		}
	};


	match &user_config.display_format[..] {
		// only display the headlines
		"h" => display::headlines(&response),
		// display the headlines and descriptions
		"h&d" => display::headline_description(&response),
		// display the headlines and the descriptions and the url
		"h&d&u" => display::headline_description_url(&response),
		// anything else
		_ => display::headline_description_url(&response),
	}
} 