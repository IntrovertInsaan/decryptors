use std::io::{self, Write};
use std::time::{Duration, Instant};
use viuer::{print_from_file, Config};

fn clear_screen() { print!("{}[2J{}[1;1H", 27 as char, 27 as char); }

fn print_ui(elapsed: Duration, attempts: u32, message: &str) {
    print!("{}[26;1H{}[J", 27 as char, 27 as char);
    println!("Time: {:.1}s | Attempts: {}", elapsed.as_secs_f64(), attempts);
    println!("Status: {}", message);
    print!("Enter your guess: ");
    io::stdout().flush().unwrap();
}

fn get_feedback(diff: i32) -> &'static str {
    if diff <= 10      { "Burning up!" }
    else if diff <= 50 { "Getting warm." }
    else               { "Cold." }
}

fn main() {
    let target: i32 = 646;
    clear_screen();

    let conf = Config { width: Some(50), height: Some(20), use_iterm: true, ..Default::default() };
    let _ = print_from_file("assets/level0.png", &conf);
    println!("==================== DECRYPTORS ====================");
    println!("  LEVEL 0 — The Fruit Cipher");
    println!("  Hint: Guess the total price of all fruits.");
    println!("====================================================");

    let mut attempts = 0u32;
    let mut message = String::from("Ready to decrypt?");
    let mut start_time: Option<Instant> = None;

    loop {
        print_ui(start_time.map(|t| t.elapsed()).unwrap_or_default(), attempts, &message);

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        if start_time.is_none() { start_time = Some(Instant::now()); }
        attempts += 1;

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => { message = "Numbers only!".to_string(); continue; }
        };

        if guess == target {
            println!("\n CORRECT! Solved in {:.2}s.", start_time.unwrap().elapsed().as_secs_f64());
            break;
        }
        message = get_feedback((guess - target).abs()).to_string();
    }
}
