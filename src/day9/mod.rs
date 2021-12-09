use std::{
    fs,
    io::{BufRead, BufReader},
};

fn read_input() -> Vec<Vec<i32>> {
    let filename = "src/day9/input";
    let file = fs::File::open(filename).unwrap();
    let contents = BufReader::new(file);

    let mut map: Vec<Vec<i32>> = Vec::new();
    for buffer in contents.lines() {
        if let Ok(buffer) = buffer {
            let this_line = buffer
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|c| c as i32)
                .collect::<Vec<i32>>();
            map.push(this_line);
        }
    }
    map
}

pub fn run() {
    let inputs = read_input();
    let risk = calculate_risk_level(&get_low_points(&inputs));
    println!("Task1: The risk level is {}", risk);
}

fn get_low_points(map: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut lows = Vec::new();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if is_low_point(&map, (x, y)) {
                lows.push(map[x][y])
            }
        }
    }
    lows
}

fn is_low_point(map: &Vec<Vec<i32>>, idx: (usize, usize)) -> bool {
    let max_x = map.len() - 1;
    let max_y = map[0].len() - 1;

    let mut neighbours = Vec::new();

    if idx.0 > 0 {
        neighbours.push((idx.0 - 1, idx.1));
    }
    if idx.0 < max_x {
        neighbours.push((idx.0 + 1, idx.1));
    }
    if idx.1 > 0 {
        neighbours.push((idx.0, idx.1 - 1));
    }
    if idx.1 < max_y {
        neighbours.push((idx.0, idx.1 + 1));
    }

    neighbours
        .iter()
        .fold(true, |acc, (x, y)| acc && (map[*x][*y] > map[idx.0][idx.1]))
}

fn calculate_risk_level(heights: &[i32]) -> i32 {
    heights.iter().fold(0, |acc, x| acc + x + 1)
}

// TODO: This calls for an iterative solution
fn get_basin_size(map: &Vec<Vec<i32>>, start: (usize, usize)) -> i32 {
    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let mut size = 0;

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_inputs() -> Vec<Vec<i32>> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
    }

    #[test]
    fn test_get_low_points() {
        let inputs = get_inputs();
        let low_points = get_low_points(&inputs);
        assert_eq!(low_points.len(), 4);
        assert!(low_points.contains(&1));
        assert!(low_points.contains(&0));
        assert!(low_points.contains(&5));
    }

    #[test]
    fn test_is_low_point() {
        let inputs = get_inputs();
        assert_eq!(is_low_point(&inputs, (0, 1)), true);
        assert_eq!(is_low_point(&inputs, (0, 9)), true);
        assert_eq!(is_low_point(&inputs, (2, 2)), true);
        assert_eq!(is_low_point(&inputs, (4, 6)), true);
        assert_eq!(is_low_point(&inputs, (0, 0)), false);
    }

    #[test]
    fn test_calculate_risk_level() {
        let inputs = vec![1, 0, 5, 5];
        assert_eq!(calculate_risk_level(&inputs), 15);
    }
}
