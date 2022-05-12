mod fetch_data;
mod config;
mod display;


pub fn get_the_news() {
	let user_config: config::Config = config::load_config();
	let response: fetch_data::ResponseObject = fetch_data::fetch_data(&user_config);
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