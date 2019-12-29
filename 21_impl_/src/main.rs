struct Rectangle
{
	width: u32,
	height: u32
}

impl Rectangle
{
	fn print_description(&self)
	{
		println!("Rectangle: {} x {}", self.width, self.height);
	}
	
	fn is_square(&self) -> bool
	{
		self.width == self.height // Return statement is not needed if one is evaluating just one expression. Do not use semicolon in that case.
	}
}

fn main()
{
	let my_rect = Rectangle { width: 10, height: 5 };
	
	my_rect.print_description();
	
	println!("Rectangle is a square: {}", my_rect.is_square());
}
