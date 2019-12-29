fn main()
{
	let number = 14;

	match number
	{
		1 => println!("It is one!"),
		2 => println!("There are two of them!"),
		10 | 11 => println!("It is either ten or eleven."),
		12 ..= 20 => println!("It is one of twelve through twenty."), // `...` range patterns are deprecated. `..=` is an inclusive range
		_ => println!("It doesn't match!")
	}
	
	let name = "Jan Kjetil";
	
	match name
	{
		"Chris" => println!("Nice name, mate!"),
		"Jan Kjetil" => println!("Yeah good on you!"),
		_ => println!("Don't know your name.")
	}
}
