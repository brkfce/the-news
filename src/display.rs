use crate::fetch_data;


// this seems inefficient, refactor

// display the news headlines to the user
pub fn headlines(response: &fetch_data::ResponseObject) {

	let mut counter = 1;
	println!();
	
	for headline in &response.articles {

		// print title of each article fetched
		println!("{}. {}\n", counter, headline.title);
		counter += 1;
	}
}

// display headline and description to the user
pub fn headline_description(response: &fetch_data::ResponseObject) {
	
	let mut counter = 1;
	println!();
	
	for article in &response.articles {

		// print title of each article fetched
		println!("{}. {}", counter, article.title);
		println!("{}\n", article.description);
		counter += 1;
	}
}

// display headline and description and url to the user
pub fn headline_description_url(response: &fetch_data::ResponseObject) {
	
	let mut counter = 1;
	println!();
	
	for article in &response.articles {

		// print title of each article fetched
		println!("{}. {}", counter, article.title);
		println!("{}", article.description);
		println!("{}\n", article.url);
		counter += 1;
	}
}