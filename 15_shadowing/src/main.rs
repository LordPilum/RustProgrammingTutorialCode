fn main()
{
	let mut x = 10;
	
	{
		let x = 15; // Shadowed variable. Value only applies within the block.
	}
	
	let x = "X is a string.";
	println!("x is {}", x);
	
	let x = true;
	println!("x is {}", x);
}
