pub const baseUrl: &str = "http://www.elgalepin.com/linguaveneta-vec.asp";
pub const ciave: &str = "ciave";
pub const offset: &str = "offset";
pub const wordUrl: &str = "http://www.elgalepin.com/linguaveneta-detalio.asp";

pub struct Word {
	pub name: String,
	pub ID: u32,
}

pub const letters: [&str; 27] = ["a", "b", "c", "d", "dh", "e", "f", "g", "gn", "h", "i", "j", "l", "m", "n", "o", "p", "r", "s", "sh", "t", "u", "v", "x", "ç", "z", "zh"];

pub const wordsPerPage: u32 = 20;
