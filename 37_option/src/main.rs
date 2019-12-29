fn get_occupation(name: &str) -> Option<&str>
{
	match name
	{
		"Jan Kjetil" => Some("Software Developer"),
		"Michael" => Some("Dentist"),
		_ => None
	}
}

fn main()
{
	let name = String::from("Jan Kjetil");
	
	println!("Character at index 12: {}",
		match name.chars().nth(12)
		{
			Some(c) => c.to_string(),
			None => "No character at index 12!".to_string()
		});

	println!("Occupation is {}",
		match get_occupation("Jan Kjetil")
		{
			Some(o) => o,
			None => "No occupation found!"
		});
}
