#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]

mod constants;

use std::process::Command;

fn curlPage(letter: char, offset: u32) -> Vec<u8> {
	println!("{}?{}={}&{}={}", constants::baseUrl, constants::ciave, letter, constants::offset, offset);

	let scrape = Command::new("curl")
		.arg(format!("{}?ciave={}&offset={}", constants::baseUrl, letter, offset))
		.output()
		.expect("failed to execute process");

	return scrape.stdout;
}


fn main() {
	let binding = String::from_utf8(curlPage('a', 2000)).unwrap();
	let html = binding.as_str();

	let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();


	// find the navigation box at the end of the page
	
	let mut navBox = dom.get_elements_by_class_name("navBox");

	let parser = dom.parser();

	loop {
		let elem = navBox.next();

		if elem.is_none() {
			break;
		}


		let tag = elem.unwrap().get(parser).unwrap();

		println!("{}", tag.inner_text(parser))
	}
	
}
