use std::io::{self, Write};
use std::time::Instant;
use viuer::{print_from_file, Config};

const TARGET: i32 = 646;
const DYN: usize = 5;

fn flush() { io::stdout().flush().unwrap(); }

fn draw_static() {
    let p = std::env::var("TERM_PROGRAM").unwrap_or_default().to_lowercase();
    let t = std::env::var("TERM").unwrap_or_default().to_lowercase();
    let wt = std::env::var("WT_SESSION").is_ok();
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l"); flush();
    let (w, h) = if wt { (75, 32) } else { (50, 20) };
    let _ = print_from_file("assets/level0.png", &Config {
        width: Some(w), height: Some(h), truecolor: true,
        use_iterm: p.contains("wezterm") || p.contains("iterm") || p.contains("ghostty"),
        use_kitty:  t.contains("kitty"),
        use_sixel:  wt,
        ..Default::default()
    });
    if wt { println!("\n"); }
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 0 · The Fruit Cipher\x1b[0m");
    println!("\x1b[2m  Hint: Guess the total price of all fruits.  q = quit\x1b[0m");
}

fn draw_dynamic(elapsed: f64, tries: u32, msg: &str) {
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
    println!("  \x1b[93m\x1b[1m⏱ {elapsed:.1}s\x1b[0m   \x1b[95m✦ {tries} tries\x1b[0m");
    println!("  {msg}");
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
    print!("\x1b[?25h\x1b[96m  ⟩\x1b[0m "); flush();
}

fn update(elapsed: f64, tries: u32, msg: &str) {
    print!("\x1b[{}F\x1b[J", DYN); flush();
    draw_dynamic(elapsed, tries, msg);
}

fn feedback(diff: i32) -> &'static str {
    match diff {
        1..=3   => "\x1b[92mAlmost there.\x1b[0m",
        4..=15  => "\x1b[93mVery close.\x1b[0m",
        16..=60 => "\x1b[33mGetting closer.\x1b[0m",
        _       => "\x1b[90mWrong answer.\x1b[0m",
    }
}

fn main() {
    draw_static();

    let start = Instant::now();
    let mut tries = 0u32;
    let mut msg = "\x1b[2mType your answer and press Enter.\x1b[0m".to_string();
    let mut first = true;

    loop {
        if first { draw_dynamic(0.0, 0, &msg); first = false; }
        else      { update(start.elapsed().as_secs_f64(), tries, &msg); }

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        let s = buf.trim();

        if matches!(s, "q" | "quit") {
            update(start.elapsed().as_secs_f64(), tries, "\x1b[2mGoodbye.\x1b[0m");
            println!(); flush(); return;
        }

        let Ok(guess) = s.parse::<i32>() else {
            msg = "\x1b[91m⚠ Numbers only.\x1b[0m".into();
            continue;
        };
        tries += 1;

        if guess == TARGET {
            update(start.elapsed().as_secs_f64(), tries, "\x1b[92m\x1b[1m🎉 DECRYPTED!\x1b[0m");
            println!(); flush(); return;
        }
        msg = feedback((guess - TARGET).abs()).into();
    }
}
