use std::env;
use minils::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = parse(&args);

    run(path);
}