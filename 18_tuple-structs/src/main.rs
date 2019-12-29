struct Color(u8, u8, u8);

fn main()
{
	let red = Color(255, 0, 0);
	
	// red.2 = 60; // Fails. red not mutable.
	
	println!("red is {}, {}, {}", red.0, red.1, red.2);
}
