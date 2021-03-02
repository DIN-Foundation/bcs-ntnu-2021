use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

    match args.len() {
    	1 => println!("Try to pass some arguments..."),
    	2 => {
    		match args[1].parse() {
    			Ok(1) => println!("One"),
    			Ok(2) => println!("Two"),
    			Ok(3) | Ok(4) => println!("Three"),
    			Ok(5..=5000) => println!("Between 1 and 5000"),
    			n => println!("You passed {:?}", n),
    		}
    	},
    	_ => println!("Too many args"),
    }
}
