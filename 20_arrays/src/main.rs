fn main()
{
	// let numbers = [1, 2, 3, 5, 6];
	let numbers: [i32; 5] = [1, 2, 3, 5, 6];
	let numof = [2; 400];

	/*
	numbers[0]; // 1
	numbers[4]; // 6
	*/
	
	// Print array using iterator.
	for n in numbers.iter()
	{
		println!("{}", n);
	}
	
	// Print array using range.
	for i in 0 .. numbers.len()
	{
		println!("{}", numbers[i]);
	}
	
	// Prints out the array with 400 2s.
	for n in numof.iter()
	{
		println!("{}", n);
	}
}
