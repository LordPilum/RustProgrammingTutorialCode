extern crate serde_json;
extern crate serde;

use serde_json::Value as JsonValue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person
{
	name: String,
	age: u8,
	is_dev: bool
}


fn main()
{
	let json_str = r#"
		{
			"name": "Jan Kjetil",
			"age": 65,
			"is_dev": true
		}
	"#;

	// Using JsonValue
	match serde_json::from_str(json_str)
	{
		Ok(val) =>
		{
			let p: JsonValue = val;
			println!("The name is {}", p["name"].as_str().unwrap()); // Not reccomended. Does not handle potential errors of to_str.
		},
		Err(_) => println!("Sorry! Could not parse JSON :(")
	}

	// Deriving.
	match serde_json::from_str(json_str)
	{
		Ok(val) =>
		{
			let p: Person = val;
			println!("The name is {}", p.name);
			println!("The age is {}", p.age);
			println!("Are they a developer? {}", p.is_dev);
		},
		Err(_) => println!("Sorry! Could not parse JSON :(")
	}
}
