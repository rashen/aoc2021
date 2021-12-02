use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        _ => {}
    }
}
