use std::fs;

pub mod problem_01;
pub mod problem_02;

pub fn solve() {
  let text = fs::read_to_string("src/days/day_04/input.txt").expect("Cannot read input file for day 4");
  let ids: Vec<&str> = text.split("\n\n").collect();

  problem_01::solve(&ids);
  problem_02::solve(&ids);
}

