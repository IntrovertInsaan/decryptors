mod engine;
mod levels;

fn main() {
    if levels::level0::run() {
        if levels::level1::run() {
            levels::level2::run();
        }
    }
}
