use std::{fs, io::BufRead, io::BufReader};

type Vec2 = [u32; 2];

#[derive(Debug, Default, Clone)]
struct Line {
    from: Vec2,
    to: Vec2,
}

impl Line {
    fn new(to: Vec2, from: Vec2) -> Self {
        Self { to, from }
    }

    fn from_raw(vals: &[u32]) -> Self {
        Self {
            from: [vals[0], vals[1]],
            to: [vals[2], vals[3]],
        }
    }
}

fn read_input() -> Vec<Line> {
    let filename = "src/day5/input";
    let file = fs::File::open(filename).unwrap();
    let contents = BufReader::new(file);

    let mut parsed_input: Vec<Line> = Vec::new();
    for l in contents.lines() {
        if let Ok(result) = l {
            let values = result
                .split(|c: char| !c.is_numeric())
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>();

            if !values.is_empty() {
                parsed_input.push(Line::from_raw(&values));
            }
        }
    }
    assert_eq!(parsed_input.len(), 500);
    parsed_input
}

fn filter_to_axis_aligned(all_lines: &Vec<Line>) -> Vec<Line> {
    all_lines
        .iter()
        .filter(|line| line.from[0] == line.to[0] || line.from[1] == line.to[1])
        .cloned()
        .collect::<Vec<Line>>()
}

pub fn run() {
    let input = read_input();
}

mod tests {
    use super::*;

    fn get_input() -> Vec<Line> {
        vec![
            Line::from_raw(&[0, 9, 5, 9]),
            Line::from_raw(&[8, 0, 0, 8]),
            Line::from_raw(&[9, 4, 3, 4]),
            Line::from_raw(&[2, 2, 2, 1]),
            Line::from_raw(&[7, 0, 7, 4]),
            Line::from_raw(&[6, 4, 2, 0]),
            Line::from_raw(&[0, 9, 2, 9]),
            Line::from_raw(&[3, 4, 1, 4]),
            Line::from_raw(&[0, 0, 8, 8]),
            Line::from_raw(&[5, 5, 8, 2]),
        ]
    }

    #[test]
    fn test_filter_to_axis_aligned() {
        let inputs = get_input();
        assert_eq!(inputs.len(), 10);
        let filtered = filter_to_axis_aligned(&inputs);
        assert_eq!(filtered.len(), 6);
    }
}
