#[cfg(test)]
mod dcode_tests
{
	#[test]
	#[ignore]
	fn test_basic1()
	{
		assert!(1 == 1); // OK
		panic!("Oh no!"); // Fails the test.
	}
	
	#[test]
	#[should_panic]
	fn test_basic2()
	{
		assert!(1 == 1); // OK
		panic!("Oh no!"); // Expected panic.
	}
	
	#[test]
	fn test_equals()
	{
		assert_eq!(super::give_two(), 1 + 1);
		assert_ne!(super::give_two(), 1 + 2);
	}
	
	#[test]
	#[should_panic]
	fn test_structs1()
	{
		let r = super::Rectangle
		{
			width: 50,
			height: 25
		};
		
		assert!(r.is_square());
	}
	
	#[test]
	fn test_structs2()
	{
		let r = super::Rectangle
		{
			width: 50,
			height: 50
		};
		
		assert!(r.is_square());
	}
}

struct Rectangle
{
	width: u8,
	height: u8
}

impl Rectangle
{
	fn is_square(&self) -> bool
	{
		self.width == self.height
	}
}

fn give_two() -> i32
{
	2
}


fn main()
{
	
}
