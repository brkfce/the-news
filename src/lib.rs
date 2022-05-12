mod fetch_data;
mod config;
mod display;


pub fn get_the_news() {
	let user_config: config::Config = config::load_config();
	let response: fetch_data::ResponseObject = fetch_data::fetch_data(user_config);
	display::headlines(response);
}