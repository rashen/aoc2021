use std::{fs, io::BufRead, io::BufReader, ops::RangeInclusive};

type Vec2 = [u32; 2];

#[derive(Debug, Default, Clone)]
struct Line {
    from: Vec2,
    to: Vec2,
}

impl Line {
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

fn get_range(x1: u32, x2: u32) -> RangeInclusive<u32> {
    RangeInclusive::new(u32::min(x1, x2), u32::max(x1, x2))
}

// Assume only diagonal and axis aligned lines
fn get_points_on_line(from: Vec2, to: Vec2) -> Vec<Vec2> {
    let direction = (to[0] as f32 - from[0] as f32) / from[1] as f32 - from[1] as f32;

    let mut points = vec![from];
    while points[points.len()] != to {
        let previous_point = points[points.len() - 1];
        let new_point: Vec2 = [0, 0]; // TODO: Get the real value
        points.push(new_point);
    }
    points
}

fn calculate_collision_grid(lines: &Vec<Line>, size: Vec2) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![0_u32; size[1] as usize]; size[0] as usize];
    for l in lines.iter() {
        // TODO: This will only work for axis aligned lines
        for i in get_range(l.from[0], l.to[0]) {
            for j in get_range(l.from[1], l.to[1]) {
                grid[i as usize][j as usize] += 1;
            }
        }
    }
    grid
}

fn filter_to_axis_aligned(lines: &Vec<Line>) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| line.from[0] == line.to[0] || line.from[1] == line.to[1])
        .cloned()
        .collect::<Vec<Line>>()
}

fn filter_to_diagonal_lines(lines: &Vec<Line>) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| {
            let min_x = u32::min(line.from[0], line.to[0]);
            let max_x = u32::max(line.from[0], line.to[0]);
            let min_y = u32::min(line.from[1], line.to[1]);
            let max_y = u32::max(line.from[1], line.to[1]);
            max_x - min_x == max_y - min_y
        })
        .cloned()
        .collect::<Vec<Line>>()
}

fn count_collisions(grid: &Vec<Vec<u32>>, limit: u32) -> u32 {
    grid.iter().flatten().fold(0, |acc, x| {
        if x >= &limit {
            return acc + 1;
        }
        acc
    })
}

pub fn run() {
    let input = read_input();
    let mut filtered_lines = filter_to_axis_aligned(&input);
    let collisions_grid = calculate_collision_grid(&filtered_lines, [1000, 1000]);
    let collisions = count_collisions(&collisions_grid, 2);
    println!(
        "Task1: The number of points where 2 lines overlap: {}",
        collisions
    );

    let mut diagonal_lines = filter_to_diagonal_lines(&input);
    filtered_lines.append(&mut diagonal_lines);
    let collisions_grid = calculate_collision_grid(&filtered_lines, [1000, 1000]);
    let collisions = count_collisions(&collisions_grid, 2);
    println!(
        "Task2: The number of points where 2 lines overlap: {}",
        collisions
    );
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

    #[test]
    fn test_calculate_collision_grid() {
        let inputs = get_input();
        let filtered = filter_to_axis_aligned(&inputs);
        let collision_grid = calculate_collision_grid(&filtered, [10, 10]);
        assert_eq!(collision_grid[5][9], 1);
        assert_eq!(collision_grid[6][9], 0);
        assert_eq!(collision_grid[0][9], 2);
        assert_eq!(collision_grid[1][9], 2);
        assert_eq!(collision_grid[2][2], 1);
        assert_eq!(collision_grid[2][1], 1);
        assert_eq!(collision_grid[0][0], 0);
    }

    #[test]
    fn test_count_collisions() {
        let grid = vec![vec![0, 0, 2, 1, 0], vec![2, 0, 0, 1, 2]];
        assert_eq!(count_collisions(&grid, 2), 3);
    }

    #[test]
    fn test_filter_to_diagonal_lines() {
        let inputs = get_input();
        let filtered = filter_to_diagonal_lines(&inputs);
        assert_eq!(filtered.len(), 4);
    }

    #[test]
    fn test_count_collisions_with_diagonal_lines() {
        let inputs = get_input();
        let mut filtered = filter_to_axis_aligned(&inputs);
        let mut diagonal_lines = filter_to_diagonal_lines(&inputs);
        filtered.append(&mut diagonal_lines);
        let collision_grid = calculate_collision_grid(&filtered, [10, 10]);
        let count = count_collisions(&collision_grid, 2);
        assert_eq!(count, 12);
    }

    #[test]
    fn test_get_points_on_line() {
        let inputs = get_input();
        let points = get_points_on_line(inputs[3].from, inputs[3].to);
        assert_eq!(points, vec![[2, 1], [2, 2]]);
        let points = get_points_on_line(inputs[9].from, inputs[9].to);
        assert_eq!(points, vec![[5, 5], [6, 4], [7, 3], [8, 2]]);
    }
}
