extern crate reqwest;

fn main()
{
	let url = "https://youtube.local/hello";

	// The short way:
	let response_text = reqwest::get(url)
		.expect("Couldn't make request.")
		.text().expect("Could not read response text!");

	println!("Response Text: {}", response_text);

	// The long (comprehensive) way:
	match reqwest::get(url)
	{
		Ok(mut response) => 
		{
			// Check of 200 OK.
			if response.status() == reqwest::StatusCode::OK
			{
				match response.text()
				{
					Ok(text) => println!("Response Text: {}", text),
					Err(_) => println!("Coiuld not read response text!")
				}
			}
			else
			{
				println!("Response was not 200 OK.");
			}
		},
		Err(_) => println!("Could not make the request!")
	}
}
