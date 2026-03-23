use std::io;
use std::time::Instant;
use crate::engine::{flush, show_completion, hash_answer};

const AMBIENT: &[u8] = include_bytes!("../../assets/level5_ambient.mp3");
const TROPHY:  &[u8] = include_bytes!("../../assets/trophies/level5_trophy.pdf");
const TARGET_HASH: &str = "bd826f81f9890e72194937a43ecce7a57f29ae67676b14c86470db8ecdfd0ac9";

const HINT: &str = "Pinpoint your strength, and then use them as opposite opportunities to grow and never stop looking up.";

const WORDS: &[&str] = &[
    "Evrp",
    "Phmrn",
    "Dzhizd",
    "Ghvkzwfy",
    "Zrulh",
    "Hmvsgz",
    "Zmlovxizy",
    "Mlyhro",
    "Vigzsx zo",
    "Ghvkzwfy",
];

fn play_ambient() -> Option<std::process::Child> {
    let mut tmp = std::env::temp_dir();
    tmp.push("decryptors_ambient.mp3");
    std::fs::write(&tmp, AMBIENT).ok();
    let path = tmp.to_str().unwrap().to_string();
    let players: &[(&str, &[&str])] = &[
        ("mpv",    &["--no-video", "--really-quiet", "--loop"]),
        ("ffplay", &["-nodisp", "-autoexit", "-loglevel", "quiet"]),
        ("vlc",    &["--intf", "dummy", "--play-and-exit"]),
        ("mplayer",&["-really-quiet"]),
    ];
    for (player, args) in players {
        let mut cmd = std::process::Command::new(player);
        cmd.args(*args).arg(&path)
           .stdout(std::process::Stdio::null())
           .stderr(std::process::Stdio::null());
        if let Ok(child) = cmd.spawn() { return Some(child); }
    }
    None
}

fn save_trophy() {
    std::fs::create_dir_all("trophies").ok();
    std::fs::write("trophies/level5_trophy.pdf", TROPHY).ok();
}

fn draw_puzzle(audio_playing: bool, msg: &str) {
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l");
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 5 · The Cosmos Cipher\x1b[0m  {}",
        if audio_playing { "\x1b[92m[p+↵] stop audio\x1b[0m" } else { "\x1b[2m[p+↵] audio\x1b[0m" });
    println!();
    println!("\x1b[2m  Hint: \"{HINT}\"\x1b[0m");
    println!();
    println!("\x1b[90m  · · · · · · · · · · · · · · · · · · · · · · ·\x1b[0m");
    println!();
    for word in WORDS {
        println!("  \x1b[93m★\x1b[0m  {word}");
    }
    println!();
    println!("\x1b[90m  · · · · · · · · · · · · · · · · · · · · · · ·\x1b[0m");
    println!();
    println!("  {msg}");
    println!();
    print!("\x1b[2m  Type your answer and press Enter  ·  q quit\x1b[0m");
    print!("\n\x1b[96m  ⟩\x1b[0m "); print!("\x1b[?25h"); flush();
}

pub fn run() -> bool {
    let mut audio: Option<std::process::Child> = None;
    let mut audio_playing = false;
    let start = Instant::now();
    let mut tries = 0u32;
    let mut msg = String::new();

    loop {
        draw_puzzle(audio_playing, &msg);

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).ok();
        let s = buf.trim();

        match s {
            "q" | "quit" => {
                if let Some(ref mut c) = audio { c.kill().ok(); }
                return false;
            }
            "p" => {
                if audio_playing {
                    if let Some(ref mut c) = audio { c.kill().ok(); audio = None; }
                    audio_playing = false;
                } else {
                    audio = play_ambient();
                    audio_playing = audio.is_some();
                    if !audio_playing {
                        msg = "\x1b[91m  install mpv for audio\x1b[0m".into();
                    }
                }
            }
            _ => {
                tries += 1;
                if hash_answer(s, "d3cr_l5") == TARGET_HASH {
                    if let Some(ref mut c) = audio { c.kill().ok(); }
                    let elapsed = start.elapsed().as_secs_f64();
                    save_trophy();
                    show_completion(
                        "LEVEL 5", "The Cosmos Cipher has been decoded.",
                        elapsed, tries,
                        "Level 5 Trophy", "trophies/level5_trophy.pdf",
                        "continue to Level 6",
                    );
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).ok();
                    if buf.trim() == "q" { return false; }
                    return true;
                }
                msg = match tries % 4 {
                    0 => "\x1b[90m✦ The stars remain silent...\x1b[0m",
                    1 => "\x1b[90m✦ Look deeper into the night sky...\x1b[0m",
                    2 => "\x1b[90m✦ The cosmos keeps its secret...\x1b[0m",
                    _ => "\x1b[90m✦ Wrong answer. Try again.\x1b[0m",
                }.into();
            }
        }
    }
}
