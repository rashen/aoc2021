use std::{fs, io::Read};

fn read_input() -> Vec<i32> {
    let filename = "src/day7/input";
    let mut file = fs::File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input");
    let values = contents
        .split(',')
        .filter_map(|r| r.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    assert_eq!(values.last().unwrap(), &947);
    values
}

fn get_fuel_cost_for_position_task1(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter().fold(0, |acc, c| acc + (pos - c).abs())
}

fn calculate_cost_task2(from: i32, to: i32) -> i32 {
    let distance = (from - to).abs();
    let mut cost = 0;
    for i in 0..=distance {
        cost += i;
    }
    cost
}

fn get_fuel_cost_for_position_task2(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs
        .iter()
        .fold(0, |acc, c| acc + calculate_cost_task2(*c, pos))
}

fn find_lowest_fuel_cost_task1(crabs: &Vec<i32>) -> i32 {
    let min = 0;
    let max = *crabs.iter().max().unwrap();

    let mut lowest_fuel_cost = i32::MAX;
    for pos in min..=max {
        let fuel_cost_for_pos = get_fuel_cost_for_position_task1(&crabs, pos);
        if fuel_cost_for_pos < lowest_fuel_cost {
            lowest_fuel_cost = fuel_cost_for_pos;
        }
    }
    lowest_fuel_cost
}

fn find_lowest_fuel_cost_task2(crabs: &Vec<i32>) -> i32 {
    let min = 0;
    let max = *crabs.iter().max().unwrap();

    let mut lowest_fuel_cost = i32::MAX;
    for pos in min..=max {
        let fuel_cost_for_pos = get_fuel_cost_for_position_task2(&crabs, pos);
        if fuel_cost_for_pos < lowest_fuel_cost {
            lowest_fuel_cost = fuel_cost_for_pos;
        }
    }
    lowest_fuel_cost
}

pub fn run() {
    let crabs = read_input();
    let lowest_fuel_cost = find_lowest_fuel_cost_task1(&crabs);
    println!("Task1: Lowest fuel cost to align: {}", lowest_fuel_cost);
    let lowest_fuel_cost = find_lowest_fuel_cost_task2(&crabs);
    println!("Task2: Lowest fuel cost to align: {}", lowest_fuel_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_inputs() -> Vec<i32> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn test_get_fuel_cost_for_position_task1() {
        assert_eq!(get_fuel_cost_for_position_task1(&get_inputs(), 2), 37);
    }

    #[test]
    fn test_find_lowest_fuel_cost_task1() {
        assert_eq!(find_lowest_fuel_cost_task1(&get_inputs()), 37);
    }

    #[test]
    fn test_calculate_cost_task2() {
        assert_eq!(calculate_cost_task2(0, 0), 0);
        assert_eq!(calculate_cost_task2(1, 5), 10);
        assert_eq!(calculate_cost_task2(16, 5), 66);
        assert_eq!(calculate_cost_task2(7, 5), 3);
        assert_eq!(calculate_cost_task2(14, 5), 45);
    }

    #[test]
    fn test_find_lowest_fuel_cost_task2() {
        assert_eq!(find_lowest_fuel_cost_task2(&get_inputs()), 168);
    }
}
