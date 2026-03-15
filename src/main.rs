mod engine;
mod levels;

fn main() {
    if levels::level0::run() {
        levels::level1::run();
    }
}
