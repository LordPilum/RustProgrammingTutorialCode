mod jkm
{
	fn chicken()
	{
		println!("Chicken!");
	}

	pub fn print_message()
	{
		chicken();
		println!("How's it going!");
	}
	
	pub mod water
	{
		pub fn print_message()
		{
			println!("I'm water!");
		}
	}
}

fn main()
{
	jkm::print_message();
	jkm::water::print_message();
}
