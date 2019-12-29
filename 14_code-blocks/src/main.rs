fn main()
{
	let x = 10;

	{
		// A code block is an isolated (from the outside)/inner scope.
		
		let y = 5;
		
		println!("x: {} y: {}", x, y);
	}

		// println!("x: {} y: {}", x, y); // Reference error for `y`
}
