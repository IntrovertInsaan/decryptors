use std::io;
use std::time::Instant;
use crate::engine::{flush, draw_dynamic, update, show_completion, hash_answer};

const TROPHY: &[u8] = include_bytes!("../../assets/trophies/level7_trophy.pdf");
const TARGET_HASH: &str = "c610c36263aad5d4fb7ff6172ebc43e3ed85a11b3f687fd94c2842f4c830c724";
const HINT: &str = "If life is a binary code, men may represent the ones and women\n  the zeros, but it takes both to create a meaningful program.";
const P: &str = "3aa0b7cc38cfa21/erahs/moc.panselzzup//:sptth";

fn decode_p() -> String {
    P.chars().rev().collect()
}

fn annotate(target: &str, text: &str) -> String {
    format!("\x1b]8;;{target}\x1b\\{text}\x1b]8;;\x1b\\")
}

fn draw_lock() -> String {
    let p  = "🧩";
    let lk = annotate(&decode_p(), p);
    let s  = "  ";

    format!("
{s}{s}{s}{p}{p}{p}{p}{p}{p}\n\
{s}{s}{s}{p}{s}{s}{s}{s}{p}\n\
{s}{p}{p}{p}{p}{p}{p}{p}{p}{p}{p}\n\
{s}{p}{s}{s}{s}{s}{s}{s}{s}{s}{p}\n\
{s}{p}{s}{s}{s}{s}{s}{s}{s}{s}{p}\n\
{s}{p}{s}{s}{s}{p}{p}{s}{s}{s}{p}\n\
{s}{p}{s}{s} {p}{lk}{p} {s}{s}{p}\n\
{s}{p}{s}{s}{s}{p}{p}{s}{s}{s}{p}\n\
{s}{p}{s}{s}{s}{s}{s}{s}{s}{s}{p}\n\
{s}{p}{p}{p}{p}{p}{p}{p}{p}{p}{p}"
    )
}

fn save_trophy() {
    std::fs::create_dir_all("trophies").ok();
    std::fs::write("trophies/level7_trophy.pdf", TROPHY).ok();
}

pub fn run() -> bool {
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l"); flush();

    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 7 · The Binary Lock\x1b[0m");
    println!();
    println!("\x1b[2m  Hint:\x1b[0m");
    println!("\x1b[2m  \"{HINT}\"\x1b[0m");
    println!();
    println!("{}", draw_lock());
    println!();
    println!("\x1b[2m  Explore carefully.\x1b[0m");

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

        if hash_answer(s, "d3cr_l7") == TARGET_HASH {
            update(start.elapsed().as_secs_f64(), tries, "\x1b[92m\x1b[1m🎉 DECRYPTED!\x1b[0m");
            flush();
            save_trophy();
            show_completion(
                "LEVEL 7", "The Binary Lock has been decoded.",
                start.elapsed().as_secs_f64(), tries,
                "Level 7 Trophy", "trophies/level7_trophy.pdf",
                "continue to Level 8",
            );
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).ok();
            if buf.trim() == "q" { return false; }
            return true;
        }

        msg = "\x1b[90mWrong answer.\x1b[0m".into();
    }
}
