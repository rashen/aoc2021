use std::{fs, io::BufRead, io::BufReader};

#[derive(Debug, Default, Clone)]
struct Pattern {
    input: Vec<String>,
    output: Vec<String>,
}

fn read_input() -> Vec<Pattern> {
    let filename = "src/day8/input";
    let file = fs::File::open(filename).unwrap();
    let contents = BufReader::new(file);
    contents
        .lines()
        .filter_map(|line| match line {
            Ok(s) => parse_line(&s),
            Err(_) => None,
        })
        .collect()
}

fn parse_line(s: &String) -> Option<Pattern> {
    let left = s.split('|').nth(0)?;
    let right = s.split('|').nth(1)?;

    Some(Pattern {
        input: left
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| String::from(s))
            .collect(),
        output: right
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| String::from(s))
            .collect(),
    })
}

fn count_ones_in_output(patterns: &Vec<Pattern>) -> u32 {
    let unique_length_of_one = 2;
    count_outputs_with_length(patterns, unique_length_of_one)
}

fn count_fours_in_output(patterns: &Vec<Pattern>) -> u32 {
    let unique_length_of_four = 4;
    count_outputs_with_length(patterns, unique_length_of_four)
}

fn count_sevens_in_output(patterns: &Vec<Pattern>) -> u32 {
    let unique_length_of_seven = 3;
    count_outputs_with_length(patterns, unique_length_of_seven)
}

fn count_eights_in_output(patterns: &Vec<Pattern>) -> u32 {
    let unique_length_of_eight = 7;
    count_outputs_with_length(patterns, unique_length_of_eight)
}

fn count_outputs_with_length(patterns: &Vec<Pattern>, length: usize) -> u32 {
    let mut n_occurences = 0;
    for p in patterns.iter() {
        for o in p.output.iter() {
            if o.chars().count() == length {
                n_occurences += 1;
            }
        }
    }
    n_occurences
}

pub fn run() {
    let patterns = read_input();

    let task1 = count_ones_in_output(&patterns)
        + count_fours_in_output(&patterns)
        + count_sevens_in_output(&patterns)
        + count_eights_in_output(&patterns);

    println!("Task1: 1, 4, 7 and 8 appear {} times in total", task1);
}
