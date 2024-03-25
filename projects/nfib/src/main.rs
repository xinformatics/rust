use std::io; 

fn main() {
    println!("nth Fibonacci number in Rust:");

    println!("Please input N");

	let mut nth = String::new(); 

	io::stdin().read_line(&mut nth).expect("Failed to read line");

	let nth: i128 = match nth.trim().parse() {
		Ok(num) => num,
		Err(_) => return,
	};
	let nfibnum = nfib(nth);

	println!("The Nth fibonacci number is {nfibnum}");

}

fn nfib(n: i128) -> i128 {

	if n == 1 {
        0 // Base case
    } else if n==2{
        1 // Base case
    } else {
        nfib(n-1) + nfib(n-2) // Recursive case
    }

}
