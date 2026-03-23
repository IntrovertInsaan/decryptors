fn main() {
    let trophies = [
        "assets/trophies/level0_trophy.pdf",
        "assets/trophies/level1_trophy.pdf",
        "assets/trophies/level2_trophy.pdf",
        "assets/trophies/level3_trophy.pdf",
        "assets/trophies/level4_trophy.pdf",
        "assets/trophies/level5_trophy.pdf",
        "assets/trophies/level6_trophy.pdf",
        "assets/trophies/level7_trophy.pdf",
    ];
    std::fs::create_dir_all("assets/trophies").ok();
    for path in &trophies {
        if !std::path::Path::new(path).exists() {
            std::fs::write(path, b"").ok();
        }
    }
}
