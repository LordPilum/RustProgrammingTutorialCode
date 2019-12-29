extern crate regex;
use regex::Regex;

fn main()
{
	let re = Regex::new(r"(\w{5})").unwrap(); // Not expecting an invalid regex. Unwrapping the result.

	let text = "dcode";

	println!("Found match? {}", re.is_match(text));

	match re.captures(text)
	{
		Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()), // We got a match so it's not going to fail at this stage. Therefore calling unwrap and returning as string.
		// Some(caps) => println!("Found match: {}", &caps[0]), // Equivalent.
		None => println!("Could not find match…")
	}
}
