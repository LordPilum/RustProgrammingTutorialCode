use std::process::Command;

fn main()
{
	// python dcode.py
	let mut cmd = Command::new("python");
	cmd.arg("dcode.py");

	// Execute the command.
	match cmd.output()
	{
		Ok(o) =>
		{
			unsafe
			{
				// Unsafe method. Does not check if string is valid UTF8.
				// One could also use the not-unchecked method but then the results need to be checked.
				println!("Output: {}", String::from_utf8_unchecked(o.stdout));
			}
		},
		Err(e) =>
		{
			println!("There was an error! {}", e);
		}
	}
}
