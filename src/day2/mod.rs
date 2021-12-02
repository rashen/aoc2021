use std::{fs, io::BufRead, io::BufReader};

#[derive(Debug, PartialEq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn read_input() -> Vec<Command> {
    let filename = "src/day2/input";
    let file = fs::File::open(filename).unwrap();
    let contents = BufReader::new(file);

    let mut parsed_input: Vec<Command> = Vec::new();
    for l in contents.lines() {
        if let Ok(result) = l {
            match parse_line(&result) {
                Some(cmd) => parsed_input.push(cmd),
                _ => {},
            }    
        }
    }
    parsed_input
}

fn parse_line(input: &str) -> Option<Command> {
    use Command::*;
    let components = input.split(' ').collect::<Vec<&str>>();
    let digit = components[1].parse::<i32>().ok()?;
    match components[0] {
        "forward" => Some(Forward(digit)),
        "down" => Some(Down(digit)),
        "up" => Some(Up(digit)),
        _ => None,
    }
}

fn calculate_destination(path: &[Command]) -> (i32, i32) {
    use Command::*;
    path.iter().fold((0,0), |(distance, depth), cmd| {
        match cmd {
            Forward(d) => (distance + d, depth),
            Down(d) => (distance, depth + d),
            Up(d) => (distance, depth - d),
        }
    })
}

fn calculate_destination_with_aim(path: &[Command]) -> (i32, i32) {
    use Command::*;
    let pos = path.iter().fold((0,0,0), |(distance, depth, aim), cmd| {
        match cmd {
            Forward(x) => (distance + x, depth + aim * x, aim),
            Down(x) => (distance, depth, aim + x),
            Up(x) => (distance, depth, aim - x),
        }
    });
    (pos.0, pos.1)
}

pub fn run() {
    let path = read_input();
    let (distance, depth) = calculate_destination(&path);
    println!("Task1: Distance travelled: {}, current depth: {}, product {}", distance, depth, distance*depth);
    let (distance, depth) = calculate_destination_with_aim(&path);
    println!("Task2: Distance travelled: {}, current depth: {}, product {}", distance, depth, distance*depth);
}

mod tests {
    use super::*;

    #[test]
    fn test_calculate_destination() {
        use Command::*;
        let path = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(calculate_destination(&path), (15, 10));
    }

    #[test]
    fn test_calculate_destination_with_aim() {
        use Command::*;
        let path = [Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        assert_eq!(calculate_destination_with_aim(&path), (15, 60));
    }

    #[test]
    fn test_parse_line() {
        use Command::*;
        assert_eq!(parse_line("forward 2"), Some(Forward(2)));
        assert_eq!(parse_line("up 10"), Some(Up(10)));
        assert_eq!(parse_line("down 22021"), Some(Down(22021)));
        assert_eq!(parse_line("dn 22021"), None);
    }


}