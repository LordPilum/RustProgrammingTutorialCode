extern crate rand;
use rand::Rng;

fn main()
{
	let random_number = rand::thread_rng().gen_range(1, 10); // Range 1 through 10.
	println!("Random Number: {}", random_number);

	// Flip a coin.
	let random_bool = rand::thread_rng().gen_weighted_bool(2); // Parameters is odds of the value being true. 1:2 in this case.
	println!("Random Boolean: {}", random_bool);
}
