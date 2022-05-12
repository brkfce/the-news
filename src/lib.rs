mod fetch_data;
mod config;
mod display;


pub fn get_the_news() {
	let user_config: config::Config = config::load_config();
	let response: fetch_data::ResponseObject = fetch_data::fetch_data(&user_config);
	match &user_config.display_format[..] {
		// only display the headlines
		"h" => display::headlines(&response),
		// display the headli ne s and descriptions
		"h&d" => display::description(&response),
		// anything else
		_ => display::headlines(&response),
	}
} 