use std::io;
use std::time::Instant;
use crate::engine::{flush, show_completion};

const DRAW_HTML: &str = include_str!("../../assets/level4_draw.html");
const TROPHY:    &[u8] = include_bytes!("../../assets/trophies/level4_trophy.pdf");
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

pub fn run() -> bool {
    open_drawing_board();
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 4 · The Shape Speaks\x1b[0m");
    println!();
    println!("\x1b[2m  Browser opened. Draw the shape, get your code.\x1b[0m");
    println!("\x1b[2m  Alt+Tab back here when ready.\x1b[0m");
    println!();
    print!("\x1b[96m  ⟩\x1b[0m "); flush();

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
