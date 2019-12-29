fn main()
{
	let explicit_vector: Vec<i32> = Vec::new();
	let mut my_vector = vec![1, 2, 3, 4];
	
	println!("{}", my_vector[2]);
	
	my_vector.push(49);
	my_vector.remove(1);
	
	for number in my_vector.iter()
	{
		println!("{}", number);
	}
}
