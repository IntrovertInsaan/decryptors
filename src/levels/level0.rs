use std::io;
use std::time::Instant;
use crate::engine::{flush, render_image, draw_dynamic, update};

const TARGET: i32 = 646;

fn feedback(diff: i32) -> &'static str {
    match diff {
        1..=3   => "\x1b[92mAlmost there.\x1b[0m",
        4..=15  => "\x1b[93mVery close.\x1b[0m",
        16..=60 => "\x1b[33mGetting closer.\x1b[0m",
        _       => "\x1b[90mWrong answer.\x1b[0m",
    }
}

pub fn run() {
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l"); flush();
    render_image("assets/level0.png");
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 0 · The Fruit Cipher\x1b[0m");
    println!("\x1b[2m  Hint: Guess the total price of all fruits.  q = quit\x1b[0m");

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
