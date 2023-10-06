#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
// HAHAHAHAHAHAHAHAHA I'll keep my parethesis
#![allow(unused_parens)]

mod constants;
mod abbrDB;

use std::process::Command;

/**
 * curl the given url / uri
 *
 * @param uri the uri / url to curl
 *
 * @return a Vec<u8> representing the stdout of curl, aka the html page
 */
fn curl(uri: String) -> String {
	let scrape = Command::new("curl").arg(uri).output().expect("failed to execute process");

	return String::from_utf8(scrape.stdout).unwrap();
}

/**
 * Each dictionary page contains 20 words the offset counts the amount of words, so, to skip a
 * page, an offset of 20 is needed
 *
 * and obviously an offset of n * 20 -> skip n pages
 *
 * also returns the amount of words added to the given array, if < 20 it means that this is the
 * last page for the given letter
 *
 * @param vec the word vector to add the words to
 * @param letera the letter the word starts with
 * @param offset the num of words to skip before retrieve more
 *
 * @return the amount of words found and added to the array 0 <= n <= 20
 */
fn getWords(arr: &mut Vec<u32>, letera: &str, offset: u32) -> u32 {
	// char the letter that the words will start with
	// offset the amount of words to skip before displaying 20 words
	let requestStr = format!("{}?{}={}&{}={}", constants::baseUrl, constants::ciave, letera, constants::offset, offset);
	let html = curl(requestStr);

	let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).unwrap();

	// find the navigation box at the end of the page

	// ?? the name ??
	let mut wordA = dom.get_elements_by_class_name("rollnarans");

	let parser = dom.parser();

	let mut count: u32 = 0;

	loop {
		let elem = wordA.next();

		if elem.is_none() {
			break;
		}

		let tag = elem.unwrap().get(parser).unwrap();

		let outH: String = tag.outer_html(parser).to_string();

		let search = "linguaveneta-detalio.asp?ID=";

		let indexStart = outH.find("linguaveneta-detalio.asp?ID=").unwrap();
		let indexEnd = outH.find("\" class=").unwrap();

		let ID = outH.get((indexStart + search.len())..indexEnd).unwrap();
		println!("{}", ID);

		arr.push(ID.parse::<u32>().unwrap());

		count += 1;
	}

	return count;
}

/**
 * calls 'getWords' for every letter in the dictionary until it reaches the end of the it
 *
 * @param arr the vector where to put all of the words retrieved from the dict
 */
fn getAllWords(arr: &mut Vec<u32>) {
	let mut offset: u32 = 0;

	for letera in constants::letters {
		loop {
			let count = getWords(arr, letera, offset);

			// less than 20 words, Next letter
			if (count < constants::wordsPerPage) {
				offset = 0;
				break;
			}

			offset += 20;
		}
	}
}

fn procuraDetaio(id: u32) { // -> Vocab {
	let uri = format!("{}?{}={}", constants::wordUrl, constants::ID, id);
	let Res = curl(uri);

}

fn main() {
	let mut wordsIds: Vec<u32> = Vec::new();

	getAllWords(&mut wordsIds);

	// println!("{:?}", wordsIds);

	// get page

	// make the deck?
}
