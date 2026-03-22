use std::io;
use crate::engine::flush;

const DRAW_HTML: &str = include_str!("../../assets/level4_draw.html");

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

pub fn run() -> bool {
    open_drawing_board();
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 4 · The Shape Speaks\x1b[0m");
    println!();
    println!("\x1b[2m  Browser opened. Draw the shape, get your code.\x1b[0m");
    println!("\x1b[2m  Alt+Tab back here when ready.\x1b[0m");
    println!();
    print!("\x1b[96m  ⟩\x1b[0m "); flush();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    let s = buf.trim().to_uppercase();

    if s == "Q" || s == "QUIT" { return false; }
    if s == "BOLD-4" { return true; }
    false
}
