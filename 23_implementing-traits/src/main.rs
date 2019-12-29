struct Person
{
	name: String,
	age: u8
}

impl ToString for Person
{
	fn to_string(&self) -> String
	{
		return format!("My name is {} and I am {}.", self.name, self.age);
	}
}


fn main()
{
	let jkm = Person { name: String::from("Jan Kjetil"), age: 37 };

	println!("{}", jkm.to_string());
}
