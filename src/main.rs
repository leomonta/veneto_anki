mod constants;

use std::process::Command;


fn main() {
	let scrape = Command::new("curl")
		.arg(constants::constants::baseUrl)
		.output()
		.expect("failed to execute process");
	
	let str_ = String::from_utf8(scrape.stdout).unwrap();

	println!("{}", str_.as_str());
}
