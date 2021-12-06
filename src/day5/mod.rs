use std::{fs, io::BufRead, io::BufReader};

type Vec2 = [u32; 2];

#[derive(Debug)]
struct Line {
    from: Vec2,
    to: Vec2,
}

fn read_input() -> Vec<Line> {
    let filename = "src/day5/input";
    let file = fs::File::open(filename).unwrap();
    let contents = BufReader::new(file);

    let mut parsed_input: Vec<Line> = Vec::new();
    for l in contents.lines() {
        if let Ok(result) = l {
            let line_def = result.split(" -> ").collect::<Vec<&str>>();
            if !line_def.is_empty() {
                let from = line_def[0]
                    .split(',')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>();
                let to = line_def[1]
                    .split(',')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<Vec<u32>>();

                parsed_input.push(Line {
                    from: from.try_into().unwrap(),
                    to: to.try_into().unwrap(),
                });
            }
        }
    }

    parsed_input
}

pub fn run() {
    let input = read_input();
}

mod tests {
    use super::*;
}
