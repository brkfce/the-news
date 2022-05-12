use reqwest;
use tokio;
use serde::Deserialize;
use crate::config;


// the object returned by the newsapi
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct response_object {
	status: String,
	total_results: i32,
	articles: Vec<article_object>
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct article_object {
	source: article_source_object,
	author: String,
	title: String,
	description: String,
	url: String,
	url_to_image: String,
	published_at: String,
	content: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct article_source_object {
	id: String,
	name: String
}


// entry point to module
pub fn fetch_data(configuration: config::Config) {

}


// construct the query string based on the user config
fn constuct_query_string(configuration: config::Config) -> String {

	// this app gets the top headlines from a source
	let url = "https://newsapi.org/v2/top-headlines?";

	// put the relevent strings together
	let query_string = format!("{}{}&{}", url, configuration.source, configuration.api_key);

	query_string
}

