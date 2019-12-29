fn main()
{
	let mut x = 10;
	
	
	let xr = &x;
	
	println!("x is {}", xr); 
	
	// xr += 1; // Fails. Not mutable.
	
	{
		// Mutable reference works.
		// Asterisk to dereference.
		// Having boh a mutable and immutable reference in the same scope is not allowed.
		let xmr = &mut x;
		*xmr += 1;
		
		println!("x is {}", xmr);
	}
	
	// println!("x is {}", xr); // Fails due to both mutable and immutable borrowings having been done.
	println!("x is {}", x); // See Rust's rules for borrowings.
}
