use std::io::{self, Write};

fn main() {
    let target: i32 = 646;

    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    println!("==================== DECRYPTORS ====================");
    println!("  LEVEL 0 — The Fruit Cipher");
    println!("  Hint: Guess the total price of all fruits.");
    println!("====================================================");

    loop {
        print!("\nEnter your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("  That's not a number! Try again."); continue; }
        };

        if guess == target {
            println!("\n CORRECT! Proceeding to next level...");
            break;
        } else {
            let diff = (guess - target).abs();
            if diff <= 10      { println!("Burning up!"); }
            else if diff <= 50 { println!("Getting warm."); }
            else               { println!("Cold."); }
        }
    }
}
