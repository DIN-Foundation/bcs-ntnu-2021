fn main() {
	let args: Vec<String> = std::env::args().collect();

    match args.len() {
    	1 => println!("Try to pass some arguments..."),
    	2 => {
    		match args[1].parse() {
    			Ok(1) => guessing_game(),
    			Ok(2) => println!("Two"),
    			Ok(3) | Ok(4) => println!("Three"),
    			Ok(5..=5000) => println!("Between 1 and 5000"),
    			n => println!("You passed {:?}", n),
    		}
    	},
    	_ => println!("Too many args"),
    }
}


/**
 * @see https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
 */
use rand::Rng;

fn guessing_game() {
    println!("--------Guess the number--------");
    println!("Please input your guess:🙏");

    let secret_number = rand::thread_rng().gen_range(1, 1001);

    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("You have to type a number.");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        std::cmp::Ordering::Less => println!("Too small!"),
        std::cmp::Ordering::Greater => println!("Too big!"),
        std::cmp::Ordering::Equal => println!("Just right! :D"),
    }
}