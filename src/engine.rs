use std::io::{self, Write};
use sha2::{Sha256, Digest};
use viuer::{print_from_file, Config};

pub const DYN: usize = 5;

pub fn flush() { io::stdout().flush().unwrap(); }

pub fn hash_answer(input: &str, salt: &str) -> String {
    let salted = format!("{}_{}", salt, input.trim().to_lowercase());
    format!("{:x}", Sha256::digest(salted.as_bytes()))
}

pub fn hash_answer_exact(input: &str, salt: &str) -> String {
    let salted = format!("{}_{}", salt, input.trim());
    format!("{:x}", Sha256::digest(salted.as_bytes()))
}

pub fn show_completion(
    level: &str,
    subtitle: &str,
    elapsed: f64,
    tries: u32,
    trophy_name: &str,
    trophy_path: &str,
    next: &str,
) {
    print!("\x1b[2J\x1b[3J\x1b[H");
    println!();
    println!("\x1b[92m\x1b[1m  ╔═══════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[92m\x1b[1m  ║                                               ║\x1b[0m");
    println!("\x1b[92m\x1b[1m  ║   ✦ {level} DECRYPTED{}\x1b[0m", " ".repeat(29usize.saturating_sub(level.len())));
    println!("\x1b[92m\x1b[1m  ║   {subtitle}{}\x1b[0m", " ".repeat(44usize.saturating_sub(subtitle.len())));
    println!("\x1b[92m\x1b[1m  ║                                               ║\x1b[0m");
    println!("\x1b[92m\x1b[1m  ║   ⏱  {elapsed:.2}s  ·  ✦ {tries} tries\x1b[0m");
    println!("\x1b[92m\x1b[1m  ║                                               ║\x1b[0m");
    println!("\x1b[92m\x1b[1m  ║   🏆 Trophy: {trophy_name}{}\x1b[0m", " ".repeat(32usize.saturating_sub(trophy_name.len())));
    println!("\x1b[92m\x1b[1m  ║   Saved → {trophy_path}{}\x1b[0m", " ".repeat(36usize.saturating_sub(trophy_path.len())));
    println!("\x1b[92m\x1b[1m  ║                                               ║\x1b[0m");
    println!("\x1b[92m\x1b[1m  ╚═══════════════════════════════════════════════╝\x1b[0m");
    println!();
    print!("\x1b[2m  Press Enter to {next}...\x1b[0m");
    flush();
}

pub fn terminal_detect() -> (bool, bool, bool) {
    let p = std::env::var("TERM_PROGRAM").unwrap_or_default().to_lowercase();
    let t = std::env::var("TERM").unwrap_or_default().to_lowercase();
    let wt = std::env::var("WT_SESSION").is_ok();
    let iterm = p.contains("wezterm") || p.contains("iterm") || p.contains("ghostty");
    let kitty = t.contains("kitty");
    (iterm, kitty, wt)
}

pub fn render_image(image_bytes: &[u8], ext: &str) {
    let (iterm, kitty, wt) = terminal_detect();
    let (w, h) = if wt { (75, 32) } else { (50, 20) };

    let mut tmp = std::env::temp_dir();
    tmp.push(format!("decryptors_img.{ext}"));
    std::fs::write(&tmp, image_bytes).ok();

    let _ = print_from_file(&tmp, &Config {
        width: Some(w), height: Some(h), truecolor: true,
        use_iterm: iterm,
        use_kitty: kitty,
        use_sixel: wt,
        ..Default::default()
    });

    std::fs::remove_file(&tmp).ok();
    if wt { println!("\n"); }
}

pub fn draw_dynamic(elapsed: f64, tries: u32, msg: &str) {
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
    println!("  \x1b[93m\x1b[1m⏱ {elapsed:.1}s\x1b[0m   \x1b[95m✦ {tries} tries\x1b[0m");
    println!("  {msg}");
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
    print!("\x1b[?25h\x1b[96m  ⟩\x1b[0m "); flush();
}

pub fn update(elapsed: f64, tries: u32, msg: &str) {
    print!("\x1b[{}F\x1b[J", DYN); flush();
    draw_dynamic(elapsed, tries, msg);
}
