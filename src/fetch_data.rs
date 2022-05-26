use reqwest;
use serde::Deserialize;
use crate::config;


// the object returned by the newsapi
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseObject {
	pub status: String,
	pub total_results: i32,
	pub articles: Vec<ArticleObject>
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleObject {
	pub source: ArticleSourceObject,
	pub author: String,
	pub title: String,
	pub description: String,
	pub url: String,
	pub url_to_image: String,
	pub published_at: String,
	pub content: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleSourceObject {
	pub id: String,
	pub name: String
}


// entry point to module
pub fn fetch_data(configuration: &config::Config) -> Result<ResponseObject, &'static str> {

	let query_string = construct_query_string(configuration);

	let result_string = make_request(query_string)?;

	let response_object = parse_response(result_string)?;

	Ok(response_object)

}


// construct the query string based on the user config
fn construct_query_string(configuration: &config::Config) -> String {

	// this app gets the top headlines from a source
	let url = "https://newsapi.org/v2/top-headlines?";

	// put the relevent strings together
	let query_string = format!("{}sources={}&apiKey={}", url, configuration.source, configuration.api_key);

	query_string
}


// perform a get request on newsapi
fn make_request(query_string: String) -> Result<String, &'static str> {

	// blocking reqwest, as this is only run once 
	let response = reqwest::blocking::get(&query_string);

	let response = match response {
		Ok(response) => response,
		Err(error) => return Err("Could not get response from NewsAPI."),
	};

	let response = response.text();

	match response {
		Ok(response) => Ok(response),
		Err(error) => Err("Could not convert response to string."),
	}
}


// parse the result into the response object structs
fn parse_response(result_string: String) -> Result<ResponseObject, &'static str> {

	// deserialise result into response struct
	let response: Result<ResponseObject, serde_json::Error> = serde_json::from_str(&result_string);

	match response {
		Ok(response) => Ok(response),
		Err(error) => Err("Error parsing response from NewsAPI, potentially due to incorrect configuration settings."),
	}
}


// need to check status of response to see if query worked