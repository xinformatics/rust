use std::io; //brings the io library into scope
use rand::Rng; // random number generator
use std::cmp::Ordering; // bring ordering into scope

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is : {secret_number}");

    loop {
  
	    println!("Please input your guess:");

	    let mut guess = String::new(); 
	    // mutable string; binds a new instance of String::new (UTF-8 encoded) function to guess
	    // ::new - new is an associated function with string type
	    io::stdin()
	    	.read_line(&mut guess)
	    	.expect("Failed to read line");

	    //let guess: u32 = guess.trim().parse().expect("Please type a number!");

	    let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
            Err(_) => continue,

	    };
	    
	    println!("You guessed: {guess}" );

	    match guess.cmp(&secret_number) {
	    	Ordering::Less => println!("Too small!"),
	        Ordering::Greater => println!("Too big!"),
	        Ordering::Equal => {
	        	println!("You win!");
	        	break;
	        }
	    }
	}
}
