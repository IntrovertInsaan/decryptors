mod engine;
mod levels;

fn main() {
    if levels::level0::run() {
        if levels::level1::run() {
            if levels::level2::run() {
                levels::level3::run();
            }
        }
    }
}
