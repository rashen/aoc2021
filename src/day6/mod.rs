use std::fs;
use std::io::Read;

#[derive(Clone, Copy)]
struct Fish {
    days_left: i32, // Until reproduction
}

impl Fish {
    fn new() -> Self {
        Self { days_left: 8 }
    }
    fn from_val(val: i32) -> Self {
        Self { days_left: val }
    }
    fn cycle(&mut self) {
        self.days_left = match self.days_left {
            0 => 6,
            _ => self.days_left - 1,
        }
    }
}

fn spawn_fish(vals: &[i32]) -> Vec<Fish> {
    let mut fishes = Vec::new();
    for v in vals {
        fishes.push(Fish::from_val(*v));
    }
    fishes
}

fn read_input() -> Vec<Fish> {
    let filename = "src/day6/input";
    let mut file = fs::File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read input");
    let values = contents
        .split(',')
        .filter_map(|r| r.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    assert_eq!(values[0], 1);
    assert_eq!(values.last().unwrap(), &1);
    spawn_fish(&values)
}

fn step(school: &mut Vec<Fish>) {
    let mut fish_to_spawn = 0;
    for fish in school.iter_mut() {
        if fish.days_left == 0 {
            fish_to_spawn += 1;
        }
        fish.cycle();
    }
    let mut new_fish = vec![Fish::new(); fish_to_spawn];
    school.append(&mut new_fish);
}

fn step_n_times(school: &mut Vec<Fish>, n: usize) {
    for _ in 0..n {
        step(school);
    }
}

pub fn run() {
    let mut school = read_input();
    step_n_times(&mut school, 80);
    println!(
        "Task1: After 80 cycles there are {} lantern fish",
        school.len()
    );
    step_n_times(&mut school, 256);
    println!(
        "Task2: After 256 cycles there are {} lantern fish",
        school.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Fish> {
        spawn_fish(&[3, 4, 3, 1, 2])
    }

    fn get_days(fishes: &[Fish]) -> Vec<i32> {
        fishes.iter().map(|f| f.days_left).collect::<Vec<i32>>()
    }

    #[test]
    fn test_step() {
        let mut school = get_input();
        step(&mut school);
        assert_eq!(get_days(&school), [2, 3, 2, 0, 1]);
        step(&mut school);
        assert_eq!(get_days(&school), [1, 2, 1, 6, 0, 8]);
        step(&mut school);
        assert_eq!(get_days(&school), [0, 1, 0, 5, 6, 7, 8]);
        step(&mut school);
        assert_eq!(get_days(&school), [6, 0, 6, 4, 5, 6, 7, 8, 8]);
    }

    #[test]
    fn test_cycle_fish() {
        let mut fish = Fish::from_val(2);
        fish.cycle();
        assert_eq!(fish.days_left, 1);
        fish.cycle();
        assert_eq!(fish.days_left, 0);
        fish.cycle();
        assert_eq!(fish.days_left, 6);
    }

    #[test]
    fn test_step_18_times() {
        let mut school = get_input();
        step_n_times(&mut school, 18);
        assert_eq!(school.len(), 26);
        assert_eq!(
            get_days(&school),
            [6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
        );
    }

    #[test]
    fn test_step_80_times() {
        let mut school = get_input();
        step_n_times(&mut school, 80);
        assert_eq!(school.len(), 5934);
    }
}
