use std::{fs, io::BufRead, io::BufReader};

fn read_input() -> Vec<i32> {
    let filename = "src/day1/input";
    let file = fs::File::open(filename).unwrap();
    let contents = BufReader::new(file);
    contents
        .lines()
        .filter_map(|r| match r {
            Ok(s) => s.parse::<i32>().ok(),
            Err(_) => None,
        })
        .collect::<Vec<i32>>()
}

fn calculate_increments(nums: &Vec<i32>, window_size: usize) -> i32 {
    nums.windows(window_size + 1).fold(0, |acc, n| {
        let first_window = &n[0..window_size];
        let second_window = &n[1..window_size + 1];

        let lhs: i32 = first_window.iter().sum();
        let rhs: i32 = second_window.iter().sum();

        match (rhs - lhs).signum() {
            1 => acc + 1,
            _ => acc,
        }
    })
}

pub fn run() {
    let depths = read_input();

    println!(
        "Part1: Number of depth increments: {}",
        calculate_increments(&depths, 1)
    );

    println!(
        "Part2: Number of depth increments: {}",
        calculate_increments(&depths, 3)
    );
}

mod test {
    use super::*;

    #[test]
    fn test_calculate_increments_window_size_1() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_increments(&depths, 1), 7);
    }

    #[test]
    fn test_calculate_increments_window_size_3() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_increments(&depths, 3), 5);
    }
}
