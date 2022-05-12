use crate::fetch_data;


// display the news headlines to the user
pub fn headlines(response: fetch_data::response_object) {

	// test value; print the number of results
	println!("The number of results is: {}", response.total_results);
}