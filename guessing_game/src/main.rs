extern crate rand;

use std::io;
use std::cmp::Ordering; 
use rand::Rng;

fn main()
{
	println!("\nGuess the number!");

	let secret_number= rand::thread_rng().gen_range(1, 101);

	//println!("The secret number is: {}", secret_number);

    loop 
    {
		println!("\nPlease input your guess");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.ok()
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse()
		{
			Ok(num) => num,
			Err(_)  => continue,
		};

		match guess.cmp(&secret_number)
		{
			Ordering::Less 	  => println!("\nToo Low, Try again!"),
			Ordering::Greater => println!("\nToo High, Try again!"),
			Ordering::Equal   => 
			{
				println!("\nYou Win!");
				break;
			}
		}
	}
}