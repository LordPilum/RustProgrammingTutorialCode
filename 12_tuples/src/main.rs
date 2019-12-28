fn main()
{
	let tup1 = (20, "Rust", 3.4, false, (1, 4, 7));
	let tup2 = (45, 6.7, "Computer");

	let (a, b, c) = tup2;

	println!("{}", (tup1.4).2);
	println!("a is {}", a);
	println!("b is {}", b);
	println!("c is {}", c);
}
