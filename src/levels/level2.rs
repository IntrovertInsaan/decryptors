use std::io;
use std::time::Instant;
use crate::engine::{flush, render_image, draw_dynamic, update, hash_answer_exact, show_completion};

const IMAGE: &[u8] = include_bytes!("../../assets/level2.png");
const TARGET_HASH: &str = "7efdba8396c71189582517c185b27626e50f4039c3395835c68b19824f468ca8";
const TROPHY_PDF: &[u8] = include_bytes!("../../assets/trophies/level2_trophy.pdf");

fn correct(guess: &str) -> bool {
    hash_answer_exact(guess, "decryptors_l2") == TARGET_HASH
}

fn save_trophy() {
    std::fs::create_dir_all("trophies").ok();
    std::fs::write("trophies/level2_trophy.pdf", TROPHY_PDF).ok();
}

pub fn run() -> bool {
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l"); flush();
    render_image(IMAGE, "png");
    println!("  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 2 · The Ludo Cipher\x1b[0m");
    println!("\x1b[2m Hint: Make Every Step Count \x1b[0m");

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
            println!(); flush(); return false;
        }

        if s.is_empty() { continue; }

        tries += 1;

        if correct(s) {
            update(start.elapsed().as_secs_f64(), tries, "\x1b[92m\x1b[1m🎉 DECRYPTED!\x1b[0m");
            flush();
            save_trophy();
            show_completion(
                "LEVEL 2",
                "The Ludo Cipher has been solved.",
                start.elapsed().as_secs_f64(), tries,
                "Periodic Table Guide",
                "trophies/level2_trophy.pdf",
                "continue to Level 3",
            );
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).ok();
            let s = buf.trim();
            if s == "q" || s == "quit" { return false; }
            return true;
        }
        msg = "\x1b[90mWrong answer.\x1b[0m".into();
    }
}
