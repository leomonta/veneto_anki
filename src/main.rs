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
 *
 * @param char the letter that the words will start with
 * @param offset the amount of words to skip before displaying 20 words
 *
 * @return a Vec<u8> representing the stdout of curl, aka the html page
 */
fn curlPage(letter: &str, offset: u32) -> Vec<u8> {
	let requestStr = format!("{}?{}={}&{}={}", constants::baseUrl, constants::ciave, letter, constants::offset, offset);

	//println!("{}", requestStr);

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
fn appendWords(html: &str, mut words: &Vec<constants::Word>) -> u32 {

	let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();

	// find the navigation box at the end of the page
	
	let mut wordA = dom.get_elements_by_class_name("rollnarans");

	let parser = dom.parser();

	let mut count: u32 = 0;

	loop {
		let elem = wordA.next();

		if elem.is_none() {
			break;
		}

		let tag = elem
			.unwrap()
			.get(parser)
			.unwrap();

		let name: String = tag.inner_text(parser).to_string();
		let outH: String = tag.outer_html(parser).to_string();

		let search = "linguaveneta-detalio.asp?ID=";

		let indexStart = outH.find("linguaveneta-detalio.asp?ID=").unwrap();
		let indexEnd = outH.find("\" class=").unwrap();
		let ID = outH.get((indexStart + search.len())..indexEnd).unwrap();
		// println!("{} -> {} {}", indexStart, indexEnd, ID);
		println!("{}", ID.parse::<u32>().unwrap());

		let word: constants::Word = constants::Word {name: name, ID: ID.parse::<u32>().unwrap()};


		count += 1;
	}

	return count;
}


fn main() {

	let mut wordsFound: Vec<constants::Word> = Vec::new();

	let mut offset: u32 = 0;
	let mut letterIndex: usize = 0;

	loop {
		let htmlOwner = String::from_utf8(curlPage(constants::letters[letterIndex], offset)).unwrap();

		let count = appendWords(htmlOwner.as_str(), &wordsFound);

		if (count < constants::wordsPerPage) {
			letterIndex += 1;
			offset = 0;
		} else {
			offset += 20;
		}

	}
	
}
