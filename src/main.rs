#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

// HAHAHAHAHAHAHAHAHA I'll keep my parethesis
#![allow(unused_parens)]

mod constants;

use std::process::Command;

/**
 * curl the dictionary page with the given letter and offset
 *
 * each dictionary page contains 20 words the offset count the amount of words, so, to skip a page
 * an offset of 20 is needed
 *
 * and obviously an offset of n*20 -> skip n pages
 */
fn curlPage(letter: char, offset: u32) -> Vec<u8> {
	let requestStr = format!("{}?{}={}&{}={}", constants::baseUrl, constants::ciave, letter, constants::offset, offset);
	println!("{}", requestStr);

	let scrape = Command::new("curl")
		.arg(requestStr)
		.output()
		.expect("failed to execute process");

	return scrape.stdout;
}

/**
 * Analyzing the dom fetches all of the words and relative pages present
 * 
 * also returns the amount of worded added to the given array, if < 20 it means that this is the
 * last page for the given letter
 *
 * @param html the html string to parse
 * @param vec the word vector to add the words to
 *
 * @return the amount of words found and added to the array 0 <= n <= 20
 */
fn appendWords(html: &str, mut vec: Vec<constants::Word>) -> u8 {

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

	return false;
}


fn main() {

	let mut array: Vec<String>;

	loop {
		let htmlOwner = String::from_utf8(curlPage('a', 2000)).unwrap();
		println!("Continuing");

		appendWords();

		if (!canContinue(htmlOwner.as_str())) {

		}
	}
	
}
