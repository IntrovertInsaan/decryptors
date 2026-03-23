mod engine;
mod levels;

fn main() {
    let passed = [
        levels::level0::run,
        levels::level1::run,
        levels::level2::run,
        levels::level3::run,
        levels::level4::run,
        levels::level5::run,
        levels::level6::run,
    ];

    for level in passed {
        if !level() { return; }
    }
}
