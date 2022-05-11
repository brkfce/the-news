mod fetch_data;
mod config;


pub fn get_the_news() {
	let user_config: config::Config = config::load_config();
}