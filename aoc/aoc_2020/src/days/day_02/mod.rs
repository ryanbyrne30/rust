use std::fs;

pub mod problem_1;
pub mod problem_2;

pub fn solve() {
  let file_path = "src/days/day_02/input.txt";
  let content = fs::read_to_string(file_path).expect("Cannot read input file for day 2");
  let lines: Vec<&str> = content.split('\n').collect();

  let mut sol_1: u32 = 0;
  let mut sol_2: u32 = 0;

  for line in lines {
    let segments: Vec<&str> = line.split(' ').collect();
    let rule = segments[0];
    let letter = segments[1].as_bytes()[0];
    let password = segments[2];
    let (a, b) = get_numbers_from_rule(rule);

    if problem_1::is_valid_password(password, letter, a, b) {
      sol_1 += 1
    }

    if problem_2::is_valid_password(password, letter, a, b) {
      sol_2 += 1
    }
  }

  println!("Day 2 - Problem 1: {}", sol_1);
  println!("Day 2 - Problem 2: {}", sol_2);
}

fn get_numbers_from_rule(rule: &str) -> (usize, usize) {
  let segments: Vec<&str> = rule.split('-').collect();
  let min: usize = segments[0].trim().parse().expect("Invalid number for rule");
  let max: usize = segments[1].trim().parse().expect("Invalid number for rule");
  return (min, max)
}