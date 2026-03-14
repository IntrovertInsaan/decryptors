use std::io::{self, Write};
use viuer::{print_from_file, Config};

pub const DYN: usize = 5;

pub fn flush() { io::stdout().flush().unwrap(); }

pub fn terminal_detect() -> (bool, bool, bool) {
    let p = std::env::var("TERM_PROGRAM").unwrap_or_default().to_lowercase();
    let t = std::env::var("TERM").unwrap_or_default().to_lowercase();
    let wt = std::env::var("WT_SESSION").is_ok();
    let iterm = p.contains("wezterm") || p.contains("iterm") || p.contains("ghostty");
    let kitty = t.contains("kitty");
    (iterm, kitty, wt)
}

pub fn render_image(path: &str) {
    let (iterm, kitty, wt) = terminal_detect();
    let (w, h) = if wt { (75, 32) } else { (50, 20) };
    let _ = print_from_file(path, &Config {
        width: Some(w), height: Some(h), truecolor: true,
        use_iterm: iterm,
        use_kitty: kitty,
        use_sixel: wt,
        ..Default::default()
    });
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
