use std::io;
use std::time::Instant;
use crate::engine::{flush, show_completion, hash_answer};
use crate::levels::colors::COLORS;

const NARRATION:     &[u8] = include_bytes!("../../assets/level3_narration.mp3");
const TROPHY:    &[u8] = include_bytes!("../../assets/trophies/level3_trophy.pdf");
const HASH_P1:  &str  = "a128d92f94482716c638d4e50c6ca8c4946f0abcebc65b3752e9cc4b1c74476f";
const HASH_P2: &str  = "92515a8d61d30f71beaef0fa5c71d752905e0ff65dde456d8ffb597dc619deb3";
const HASH_RGB_P2: &str = "f41a0341564a91344903a28c3149ab975749a47d5917743f88a2177a849994c1";

const BLOOD_EMOJI: &str = "🟥 🩸🩸🩸🧨🩸⭕ 🅰️🌹🍎🍒🍷🩸🩸🥊👠👹👺🩸🔴🩸♥️♦️♨️⛑️🩸❌🅱️💯🖍️🚨🚩🩸🩸🩸🍓🩸🍅🍣🛑🩸🩸🩸🧲🩸🧯🩸🩸🩸🔖🩸🩸📌📍🩸🔴♥️🥀
🧨⭕🩸 🅰️🌹🍎🍒🍷🥊👠👹👺🔴🩸♥️♦️♨️⛑️🩸🩸🩸❌🩸🅱️💯🩸🖍️🚨🩸🩸🩸🚩🩸🩸🍓🩸🩸🩸🍅🍣🛑🧲🧯🔖📌📍🔴♥️🥀
🧨⭕ 🅰️🩸🌹🍎🍒🍷🥊👠👹👺🔴♥️♦️♨️🩸🩸🩸⛑️❌🅱️🩸🩸💯🖍️🚨🚩🍓🍅🍣🛑🧲🧯🔖🩸📌🩸🩸🩸🩸📍🔴♥️🥀🩸
🧨⭕ 🅰️🩸🌹🍎🍒🩸🍷🥊👠👹👺🔴🩸🩸♥️♦️♨️⛑️❌🩸🅱️💯🖍️🚨🚩🍓🩸🍅🩸🩸🍣🩸🛑🧲🧯🔖📌📍🔴🩸🩸♥️🥀
🧨⭕🩸 🅰️🩸🌹🍎🩸🩸🍒🍷🥊👠👹👺🔴♥️♦️♨️⛑️❌🅱️🩸💯🖍️🚨🚩🩸🍓🍅🩸🍣🛑🧲🧯🔖📌📍🔴♥️🥀
⭕ 🅰️🌹🍎🍒🩸🍷🥊👠👹👺🩸🩸🔴♥️♦️♨️🩸🩸⛑️🩸❌🅱️🩸💯🖍️🚨🚩🍓🍅🍣🛑🧲🧯🔖🩸📌📍🔴♥️🥀
🧨🩸🩸🩸⭕ 🅰️🩸🌹🍎🍒🍷🥊👠👹👺🔴♥️♦️♨️⛑️🩸❌🅱️🩸🩸💯🖍️🚨🚩🍓🍅🍣🛑🧲🧯🔖📌🩸📍🔴♥️🥀
🧨⭕🩸🩸🩸 🅰️🩸🌹🍎🍒🍷🥊👠👹👺🔴♥️🩸🩸🩸🩸🩸🩸♦️♨️🩸🩸⛑️🩸❌🅱️💯🖍️🚨🚩🍓🍅🍣🛑🧲🧯🩸🩸🩸🩸🔖📌🩸🩸📍🔴♥️🥀
🧨⭕🩸 🅰️🩸🌹🍎🍒🍷🥊👠👹👺🔴🩸🩸🩸🩸🩸🩸♥️🩸♦️♨️⛑️🩸❌🅱️💯🖍️🚨🚩🍓🍅🍣🛑🧲🧯🔖📌📍🔴🩸🩸♥️🩸🩸🩸🩸🩸🩸🥀
🧨⭕ 🅰️🌹🩸🍎🍒🩸🩸🩸🩸🩸🩸🩸🩸🍷🥊👠👹👺🔴🩸♥️♦️♨️⛑️❌🅱️💯🖍️🚨🚩🍓🍅🍣🩸🩸🩸🛑🧲🧯🩸🔖📌📍🔴♥️🥀
🧨⭕🩸🩸🩸 🅰️🌹🩸🩸🩸🩸🩸🍎🩸🩸🍒🍷🥊👠👹👺🔴♥️♦️♨️⛑️❌🅱️💯🖍️🚨🩸🩸🩸🩸🩸🚩🍓🍅🩸🩸🩸🩸🍣🛑🧲🧯🔖📌📍🔴♥️🥀
🧨⭕🩸🩸🩸 🅰️🩸🩸🩸🌹🍎🩸🍒🍷🥊👠👹👺🩸🩸🔴♥️♦️♨️⛑️❌🅱️💯🩸🩸🩸🖍️🩸🩸🩸🩸🚨🩸🚩🍓🩸🩸🩸🍅🍣🛑🧲🩸🩸🩸🧯🩸🔖📌📍🩸🔴♥️🩸🥀
🧨⭕ 🅰️🌹🍎🍒🍷🥊👠👹🩸🩸🩸👺🔴♥️🩸🩸♦️♨️⛑️🩸🩸❌🅱️💯🖍️🚨🚩🍓🩸🩸🩸🩸🩸🩸🩸🩸🍅🍣🛑🧲🧯🔖🩸📌📍🔴♥️🥀
🧨⭕ 🩸🩸🩸🩸🩸🩸🅰️🌹🍎🩸🩸🍒🍷🥊👠👹👺🔴♥️♦️♨️⛑️❌🅱️💯🖍️🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸🩸";
const GRASS_EMOJI: &str = "💚☘️☘️☘️🌱🌿🍃☘️☘️ 🌵🌴☘️☘️☘️🌳🌲🏝️🐲🦎🐉🦖🐊🐍☘️☘️☘️🐸☘️☘️🐛🍈☘️🍏🥦🥒🥬☘️☘️🥑☘️🥗🎄🧪📗🟩☘️🟢☘️💚✳️❇️♻️💹🈯❎✅🇨🇨🇳🇫🇳🇬🇸🇦";
const WATER_EMOJI: &str = "🌐💦💦💦💦💦💦💦💦💦💦💦💦💦💦🥶💦💦💦💦💦💦💦🐬💦👖🧢💎💍💦💦🐋🥣💦💦💦💦💦💦📘💦💦💦💦💦💦💦💦💦💦💦💦🎭💦💦🚙💦💦💦💦🧊💦💦💦💦💦💤💙💦🌊💦⛵💦💦💦\n🐟🐳💦";

fn nearest_color(r: u8, g: u8, b: u8) -> &'static str {
    COLORS.iter()
        .min_by_key(|(_, cr, cg, cb)| {
            let (dr, dg, db) = (*cr as i32 - r as i32, *cg as i32 - g as i32, *cb as i32 - b as i32);
            dr*dr + dg*dg + db*db
        })
        .map(|(name, ..)| *name)
        .unwrap_or("unknown")
}

fn color_block(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{r};{g};{b}m            \x1b[0m")
}

fn to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn play_narration() -> Option<std::process::Child> {
    let mut tmp = std::env::temp_dir();
    tmp.push("decryptors_narration.mp3");
    std::fs::write(&tmp, NARRATION).ok();
    let path = tmp.to_str().unwrap().to_string();
    let players: &[(&str, &[&str])] = &[
        ("mpv",    &["--no-video", "--really-quiet"]),
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
    std::fs::write("trophies/level3_trophy.pdf", TROPHY).ok();
}

fn draw_puzzle(audio_playing: bool, show_tool: bool, r: u8, g: u8, b: u8, step: u8, elapsed: f64, tries: u32) {
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l");
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 3 · The Three Realms\x1b[0m  {}  {}",
        if audio_playing { "\x1b[92m[p+↵] stop\x1b[0m" } else { "\x1b[2m[p+↵] narration\x1b[0m" },
        if show_tool    { "\x1b[92m[c+↵] hide tool\x1b[0m" } else { "\x1b[2m[c+↵] color mixer\x1b[0m" });
    println!();
    println!("\x1b[91m\x1b[1m  ── Blood Domain 🩸──────────────────────────\x1b[0m");
    println!("  {BLOOD_EMOJI}");
    println!();
    println!("\x1b[92m\x1b[1m  ── Living Forest ☘️─────────────────────────\x1b[0m");
    println!("  {GRASS_EMOJI}");
    println!();
    println!("\x1b[94m\x1b[1m  ── Aquatic Realm 💦 ─────────────────────────\x1b[0m");
    println!("  {WATER_EMOJI}");

    if show_tool {
        println!();
        println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
        let r_bar = "█".repeat((r as usize * 20 / 255).min(20));
        let r_emp = "░".repeat(20 - (r as usize * 20 / 255).min(20));
        let g_bar = "█".repeat((g as usize * 20 / 255).min(20));
        let g_emp = "░".repeat(20 - (g as usize * 20 / 255).min(20));
        let b_bar = "█".repeat((b as usize * 20 / 255).min(20));
        let b_emp = "░".repeat(20 - (b as usize * 20 / 255).min(20));
        if step == 0 { println!("  \x1b[91m\x1b[1mR\x1b[0m {:>3}  \x1b[91m{}\x1b[90m{}\x1b[0m  \x1b[96m◀\x1b[0m", r, r_bar, r_emp); }
        else         { println!("  \x1b[91mR\x1b[0m {:>3}  \x1b[91m{}\x1b[90m{}\x1b[0m", r, r_bar, r_emp); }
        if step == 1 { println!("  \x1b[92m\x1b[1mG\x1b[0m {:>3}  \x1b[92m{}\x1b[90m{}\x1b[0m  \x1b[96m◀\x1b[0m", g, g_bar, g_emp); }
        else         { println!("  \x1b[92mG\x1b[0m {:>3}  \x1b[92m{}\x1b[90m{}\x1b[0m", g, g_bar, g_emp); }
        if step == 2 { println!("  \x1b[94m\x1b[1mB\x1b[0m {:>3}  \x1b[94m{}\x1b[90m{}\x1b[0m  \x1b[96m◀\x1b[0m", b, b_bar, b_emp); }
        else         { println!("  \x1b[94mB\x1b[0m {:>3}  \x1b[94m{}\x1b[90m{}\x1b[0m", b, b_bar, b_emp); }
        println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
        println!("  {}  \x1b[1m{}\x1b[0m  \x1b[2m{}\x1b[0m",
            color_block(r, g, b), nearest_color(r, g, b), to_hex(r, g, b));
        println!();
        if step < 3 {
            let label = ["R (0-255)", "G (0-255)", "B (0-255)"][step as usize];
            println!("\x1b[2m  Enter {} value:\x1b[0m", label);
        } else {
            println!("\x1b[2m  ⏱ {elapsed:.1}s  ·  {tries} tries  ·  Enter to confirm\x1b[0m");
        }
    } else {
        println!();
        println!("\x1b[2m  Count each potion, then open the color mixer with [c+↵]\x1b[0m");
    }
    println!();
    print!("\x1b[96m  ⟩\x1b[0m "); print!("\x1b[?25h"); flush();
}


pub fn run() -> bool {
    let mut audio: Option<std::process::Child> = None;
    let mut audio_playing = false;
    let mut show_tool = false;
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;
    let mut step: u8 = 0;
    let start = Instant::now();
    let mut tries = 0u32;

    loop {
        draw_puzzle(audio_playing, show_tool, r, g, b, step, start.elapsed().as_secs_f64(), tries);

        let mut line = String::new();
        io::stdin().read_line(&mut line).ok();
        let s = line.trim();

        match s {
            "q" | "quit" => {
                if let Some(ref mut c) = audio { c.kill().ok(); }
                print!("\x1b[2J\x1b[3J\x1b[H\n  \x1b[2mGoodbye.\x1b[0m\n"); flush();
                return false;
            }
            "p" => {
                if audio_playing {
                    if let Some(ref mut c) = audio { c.kill().ok(); audio = None; }
                    audio_playing = false;
                } else {
                    audio = play_narration();
                    audio_playing = audio.is_some();
                    if !audio_playing {
                        print!("\x1b[2J\x1b[H\n  \x1b[91m  install mpv for audio\x1b[0m\n"); flush();
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    }
                }
            }
            "c" => {
                show_tool = !show_tool;
                if !show_tool {
                    r = 0; g = 0; b = 0; step = 0;
                }
            }
            "" if show_tool && step == 3 => {
                let name = nearest_color(r, g, b);
                tries += 1;

                let rgb_str = format!("{},{},{}", r, g, b);
                if hash_answer(&rgb_str, "d3_rgb") == HASH_RGB_P2
                    && hash_answer(name, "d3_p0t10n_e") == HASH_P2
                {
                    if let Some(ref mut c) = audio { c.kill().ok(); }
                    print!("\x1b[2J\x1b[3J\x1b[H");
                    println!();
                    println!("\x1b[95m\x1b[1m  ╔═══════════════════════════════════════════════╗\x1b[0m");
                    println!("\x1b[95m\x1b[1m  ║   🔮 HIDDEN CIPHER DISCOVERED                ║\x1b[0m");
                    println!("\x1b[95m\x1b[1m  ║   You found the Ludo Color Code.             ║\x1b[0m");
                    println!("\x1b[95m\x1b[1m  ║                                               ║\x1b[0m");
                    println!("\x1b[95m\x1b[1m  ║   ⚡ ABILITY: Indigo Eyes  (1 use)           ║\x1b[0m");
                    println!("\x1b[95m\x1b[1m  ║   Reveals hidden names in future puzzles.    ║\x1b[0m");
                    println!("\x1b[95m\x1b[1m  ╚═══════════════════════════════════════════════╝\x1b[0m");
                    println!();
                    println!("\x1b[2m  [Enter] Try counting puzzle too   [s] Skip to Level 4\x1b[0m");
                    print!("\x1b[96m  ⟩\x1b[0m "); print!("\x1b[?25h"); flush();
                    let mut choice = String::new();
                    io::stdin().read_line(&mut choice).ok();
                    if choice.trim() == "s" { return true; }
                    r = 0; g = 0; b = 0; step = 0;
                }
                else if hash_answer(name, "d3_p0t10n") == HASH_P1 {
                    if let Some(ref mut c) = audio { c.kill().ok(); }
                    let elapsed = start.elapsed().as_secs_f64();
                    save_trophy();
                    show_completion(
                        "LEVEL 3", "The Three Realms has been decrypted.",
                        elapsed, tries,
                        "Level 3 Trophy", "trophies/level3_trophy.pdf",
                        "continue to Level 4",
                    );
                    println!();
                    println!("\x1b[91m\x1b[1m  ⚡ Crimson Eyes unlocked  (1 use)\x1b[0m");
                    println!("\x1b[2m  Reveals any character name in a future puzzle.\x1b[0m");
                    println!();
                    print!("\x1b[2m  Press Enter to continue...\x1b[0m");
                    print!("\x1b[?25h"); flush();
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).ok();
                    if buf.trim() == "q" { return false; }
                    return true;
                } else {
                    r = 0; g = 0; b = 0; step = 0;
                }
            }
            _ if show_tool && step < 3 => {
                if let Ok(n) = s.parse::<u16>() {
                    match step {
                        0 => r = n.min(255) as u8,
                        1 => g = n.min(255) as u8,
                        _ => b = n.min(255) as u8,
                    }
                    step += 1;
                }
            }
            _ => {}
        }
    }
}
