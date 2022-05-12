use crate::fetch_data;


// display the news headlines to the user
pub fn headlines(response: &fetch_data::ResponseObject) {

	let mut counter = 1;
	
	for headline in &response.articles {

		// print title of each article fetched
		println!("{}. {}", counter, headline.title);
		counter += 1;
	}
}

// display headline and description to the user
pub fn description(response: &fetch_data::ResponseObject) {
	
	let mut counter = 1;
	
	for article in &response.articles {

		// print title of each article fetched
		println!("{}. {}", counter, article.title);
		println!("{}", article.description);
		counter += 1;
	}
}