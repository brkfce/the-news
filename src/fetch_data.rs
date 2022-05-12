use reqwest;
use tokio;
use serde::Deserialize;
use crate::config;


// the object returned by the newsapi
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct response_object {
	pub status: String,
	pub total_results: i32,
	pub articles: Vec<article_object>
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct article_object {
	pub source: article_source_object,
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
pub struct article_source_object {
	pub id: String,
	pub name: String
}


// entry point to module
pub fn fetch_data(configuration: config::Config) -> response_object {

	let query_string = construct_query_string(configuration);

	let result_string = make_request(query_string);

	let response_object = parse_response(result_string);

	response_object

}


// construct the query string based on the user config
fn construct_query_string(configuration: config::Config) -> String {

	// this app gets the top headlines from a source
	let url = "https://newsapi.org/v2/top-headlines?";

	// put the relevent strings together
	let query_string = format!("{}{}&{}", url, configuration.source, configuration.api_key);

	query_string
}


// perform a get request on newsapi
fn make_request(query_string: String) -> String {

	// blocking reqwest, as this is only run once 
	let response = reqwest::blocking::get(&query_string).expect("Could not get response from NewsAPI").text().expect("Could not convert response to string.");

	response
}


// parse the result into the response object structs
fn parse_response(result_string: String) -> response_object {

	// deserialise result into response struct
	let response: response_object = serde_json::from_str(&result_string).expect("Error parsing response from API");

	response
}


// need to check status of response to see if query worked