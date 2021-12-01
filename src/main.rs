use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => day1::run(),
        _ => {}
    }
}
