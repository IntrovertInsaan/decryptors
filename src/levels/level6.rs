use std::io;
use std::time::Instant;
use crate::engine::{flush, render_image, draw_dynamic, update, show_completion, hash_answer};

const IMAGE: &[u8] = include_bytes!("../../assets/level6.png");
const TROPHY: &[u8] = include_bytes!("../../assets/trophies/level6_trophy.pdf");
const TARGET_HASH: &str = "7eaed9ab8f29d06772a9ef8afb2ae8b76dbcb100f56ff01fee585891451adabe";

const STORY: &str = "\
  A group of animals, including a horse, turtle, tapir, panda, and sheep set out on a journey.\n\
  Along the way, they encountered various challenges, such as a venomous snake bite and a scratching cat.\n\
  They worked together and were aided by other animals like bears, iguanas, tigers, lions, and yaks.\n\
  Together they battled with deadliest bee stings, cured by a wise cat who used herbs of xeme tree.\n\
  As they journeyed, they came across the same spot where they had fought the scratching cat,\n\
  and they saw a giant boa and Ostrich. In the path they also passed Cockroach town,\n\
  where they had to be extra careful not to fall into the Nightingale water.\n\
  Finally, the animals reached their destination, where they were greeted by a friendly Urial\n\
  and shared stories with them.";

fn save_trophy() {
    std::fs::create_dir_all("trophies").ok();
    std::fs::write("trophies/level6_trophy.pdf", TROPHY).ok();
}

pub fn run() -> bool {
    print!("\x1b[2J\x1b[3J\x1b[H\x1b[?25l"); flush();
    render_image(IMAGE, "png");
    println!("\x1b[93m\x1b[1m  ✦ DECRYPTORS\x1b[0m  \x1b[2mLevel 6 · The Fable Cipher\x1b[0m");
    println!("\x1b[2m  Hint: WHO AM I?  Find the name.\x1b[0m");
    println!();
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");
    for line in STORY.lines() {
        println!("\x1b[2m  {line}\x1b[0m");
    }
    println!("\x1b[2m                   THE END\x1b[0m");
    println!("\x1b[90m  ─────────────────────────────────────────\x1b[0m");

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

        if hash_answer(s, "d3cr_l6") == TARGET_HASH {
            update(start.elapsed().as_secs_f64(), tries, "\x1b[92m\x1b[1m🎉 DECRYPTED!\x1b[0m");
            flush();
            save_trophy();
            show_completion(
                "LEVEL 6", "The Fable Cipher has been solved.",
                start.elapsed().as_secs_f64(), tries,
                "Level 6 Trophy", "trophies/level6_trophy.pdf",
                "continue to Level 7",
            );
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).ok();
            if buf.trim() == "q" { return false; }
            return true;
        }

        msg = "\x1b[90mWrong answer.\x1b[0m".into();
    }
}
