use std::io; 

fn main() {
    println!("Temperature converter in Rust");
    tconvert();

}

fn tconvert() -> f32 {
	
	loop {
  
	    println!("Please input Temperature in Farenheit");

	    let mut ftemp = String::new(); 

	    io::stdin().read_line(&mut ftemp).expect("Failed to read line");

	    let ftemp: f32 = match ftemp.trim().parse() {
        	Ok(num) => num,
        	Err(_) => continue,
    	};
		
		let ctemp = 5.0*(ftemp - 32.0 )/9.0;

	    println!("Celsius Temperature is : {ctemp} degC." );
	    
	     
	}
	
}
