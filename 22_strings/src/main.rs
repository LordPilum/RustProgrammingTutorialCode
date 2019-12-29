fn main()
{
	let mut my_string = String::from("How is it going? My name is Jan Kjetil.");

	// Length.
	println!("Length: {}", my_string.len());

	// Is empty?.
	println!("My string is empty? {}", my_string.is_empty());

	for token in my_string.split_whitespace()
	{
		println!("{}", token);
	}
	
	println!("Does the string contain 'Jan Kjetil'? {}", my_string.contains("Jan Kjetil"));

	my_string.push_str(" Welcome to your tutorial on Strings!");

	println!("{}", my_string);
}
