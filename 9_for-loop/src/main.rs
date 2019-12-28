fn main()
{
	/*
	let numbers = 30 .. 51; // Range, [m, n>

	for i in numbers
	{
		println!("The number is {}", i);
	}
	*/
	
	let animals = vec!["Rabibt", "Dog", "Cat"];
	
	/*
	for a in animals.iter()  // Referencing the animals vector iterator.
	{
		println!("The animal name is {}", a);
	}
	*/
	
	for (index, a) in animals.iter().enumerate()  // Enumerating the vectpr
	{
		println!("The index is {} and the animal name is {}", index, a);
	}
}
