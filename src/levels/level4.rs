use std::io;
use std::time::Instant;
use crate::engine::{flush, show_completion};

const DRAW_HTML: &str = include_str!("../../assets/level4_draw.html");
const TROPHY:    &[u8] = include_bytes!("../../assets/trophies/level4_trophy.pdf");
const HINT_PDF:  &[u8] = include_bytes!("../../assets/level4_hint.pdf");
const HASH_P1:   &str  = "58c5f6030b17b71d6eec6ed12f88b3d185c842cea1b1f1707d356690f297fcc4";

fn open_drawing_board() {
    let mut tmp = std::env::temp_dir();
    tmp.push("decryptors_level4.html");
    std::fs::write(&tmp, DRAW_HTML).ok();
    std::process::Command::new("xdg-open")
        .arg(&tmp)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn().ok();
}

fn save_trophy() {
    std::fs::create_dir_all("trophies").ok();
    std::fs::write("trophies/level4_trophy.pdf", TROPHY).ok();
}

fn save_hint() {
    std::fs::create_dir_all("trophies").ok();
    std::fs::write("trophies/level4_hint.pdf", HINT_PDF).ok();
}

pub fn run() -> bool {
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l"); flush();

    // ── hint phase ────────────────────────────────────────────────────────────
    save_hint();
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 4 · The Shape Speaks\x1b[0m");
    println!();
    println!("\x1b[2m  A new file has appeared → trophies/level4_hint.pdf\x1b[0m");
    println!();
    println!("\x1b[2m  Hint: the file is locked. your past holds the key.\x1b[0m");
    println!();
    print!("\x1b[2m  [p+↵] open hint   [↵] continue to drawing   q quit\x1b[0m");
    print!("\x1b[?25h"); flush();

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        match buf.trim() {
            "q" | "quit" => return false,
            "p" => {
                std::process::Command::new("xdg-open")
                    .arg("trophies/level4_hint.pdf")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn().ok();
            }
            _ => break,
        }
    }

    // ── drawing phase ─────────────────────────────────────────────────────────
    open_drawing_board();
    print!("\x1b[2J\x1b[3J\x1b[H");
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 4 · The Shape Speaks\x1b[0m");
    println!();
    println!("\x1b[2m  Browser opened. Draw what you found, get your code.\x1b[0m");
    println!("\x1b[2m  Alt+Tab back here when ready.\x1b[0m");
    println!();
    print!("\x1b[96m  ⟩\x1b[0m "); print!("\x1b[?25h"); flush();

    let start = Instant::now();
    let mut tries = 0u32;

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        let s = buf.trim();

        if matches!(s, "q" | "quit") { return false; }

        tries += 1;

        if s == HASH_P1 {
            let elapsed = start.elapsed().as_secs_f64();
            save_trophy();
            show_completion(
                "LEVEL 4", "The Shape Speaks has been decoded.",
                elapsed, tries,
                "Level 4 Trophy", "trophies/level4_trophy.pdf",
                "continue to Level 5",
            );
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).ok();
            if buf.trim() == "q" { return false; }
            return true;
        }

        print!("\x1b[2m  Wrong code. Try again.\x1b[0m\n\x1b[96m  ⟩\x1b[0m "); flush();
    }
}
