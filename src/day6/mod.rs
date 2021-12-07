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

fn get_days(fishes: &[Fish]) -> Vec<i32> {
    fishes.iter().map(|f| f.days_left).collect::<Vec<i32>>()
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

    let init_vals = get_days(&read_input());
    println!(
        "Task2: After 256 cycles there are {} lantern fish",
        cycle_school_n_times(&init_vals, 256)
    );
}

// This approach is way faster
fn spawn_schools(init_vals: &[i32]) -> Vec<u64> {
    let mut fishes: Vec<u64> = vec![0; 9];
    for v in init_vals {
        fishes[*v as usize] += 1;
    }

    fishes
}

fn cycle_schools(fishes_per_day: Vec<u64>) -> Vec<u64> {
    let mut new_fish_per_day = vec![0; 9];
    for (idx, fish) in fishes_per_day.iter().enumerate() {
        match idx {
            0 => {
                new_fish_per_day[6] += fish;
                new_fish_per_day[8] += fish;
            }
            x => new_fish_per_day[x - 1] += fish,
        }
    }
    assert_eq!(new_fish_per_day.len(), 9);
    new_fish_per_day
}

fn cycle_school_n_times(init_vals: &[i32], n: usize) -> u64 {
    let mut schools = spawn_schools(init_vals);
    for _ in 0..n {
        schools = cycle_schools(schools);
    }
    schools.iter().fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Fish> {
        spawn_fish(&[3, 4, 3, 1, 2])
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

    #[test]
    fn test_cycle_school_18_times() {
        let init_vals = [3, 4, 3, 1, 2];
        let total_fish = cycle_school_n_times(&init_vals, 18);
        assert_eq!(total_fish, 26);
    }

    #[test]
    fn test_cycle_school_80_times() {
        let init_vals = [3, 4, 3, 1, 2];
        let total_fish = cycle_school_n_times(&init_vals, 80);
        assert_eq!(total_fish, 5934);
    }
}
