use std::io::{self, Write};
use std::time::Instant;
use viuer::{print_from_file, Config};

const TARGET: i32 = 646;

fn flush() { io::stdout().flush().unwrap(); }

fn draw_static() {
    let p = std::env::var("TERM_PROGRAM").unwrap_or_default().to_lowercase();
    let t = std::env::var("TERM").unwrap_or_default().to_lowercase();
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l"); flush();
    let _ = print_from_file("assets/level0.png", &Config {
        width: Some(50), height: Some(20), truecolor: true,
        use_iterm: p.contains("wezterm") || p.contains("iterm"),
        use_kitty:  t.contains("kitty"),
        ..Default::default()
    });
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 0 · The Fruit Cipher\x1b[0m");
    println!("\x1b[2m  Hint: Guess the total price of all fruits.  q = quit\x1b[0m");
}

fn draw(row: u16, elapsed: f64, tries: u32, msg: &str) {
    print!("\x1b[{row};1H\x1b[J");
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
    println!("  \x1b[93m\x1b[1m⏱ {elapsed:.1}s\x1b[0m   \x1b[95m✦ {tries} tries\x1b[0m");
    println!("  {msg}");
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
    print!("\x1b[?25h\x1b[96m  ⟩\x1b[0m "); flush();
}

fn feedback(diff: i32) -> &'static str {
    match diff {
        1..=10  => "\x1b[93mGetting warm.\x1b[0m",
        11..=50 => "\x1b[33mGetting closer.\x1b[0m",
        _       => "\x1b[90mWrong answer.\x1b[0m",
    }
}

fn main() {
    draw_static();

    const ROW: u16 = 26;
    let start = Instant::now();
    let mut tries = 0u32;
    let mut msg = "\x1b[2mType your answer and press Enter.\x1b[0m".to_string();

    loop {
        draw(ROW, start.elapsed().as_secs_f64(), tries, &msg);

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        let s = buf.trim();

        if matches!(s, "q" | "quit") {
            draw(ROW, start.elapsed().as_secs_f64(), tries, "\x1b[2mGoodbye.\x1b[0m");
            println!(); flush(); return;
        }

        let Ok(guess) = s.parse::<i32>() else {
            msg = "\x1b[91m⚠ Numbers only.\x1b[0m".into();
            continue;
        };
        tries += 1;

        if guess == TARGET {
            draw(ROW, start.elapsed().as_secs_f64(), tries, "\x1b[92m\x1b[1m🎉 DECRYPTED!\x1b[0m");
            println!(); flush(); return;
        }
        msg = feedback((guess - TARGET).abs()).into();
    }
}
