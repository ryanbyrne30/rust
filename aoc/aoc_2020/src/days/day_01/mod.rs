use std::fs;
use crate::algs::sorting::qsort;

pub mod problem_1;
pub mod problem_2;

const TARGET_SUM: i32 = 2020;

pub fn solve() {
  // Parse input
  let file_path = "src/days/day_01/input.txt";
  let text = fs::read_to_string(file_path).expect("Cannot read input file for day 1.");
  let words: Vec<&str> = text.split('\n').collect();
  let mut numbers: Vec<i32> = vec![];

  for word in words {
    let number: i32 = word.trim().parse().expect("Invalid number from input");
    numbers.push(number);
  }

  // Sort numbers
  qsort::qsort(&mut numbers);

  // Solution for problem 1
  let sol_1 = problem_1::solve(&numbers, TARGET_SUM);
  println!("Day 1 - Problem 1: {}", sol_1);

  // Solution for problem 2
  let sol_2 = problem_2::solve(&numbers, TARGET_SUM);
  println!("Day 1 - Problem 2: {}", sol_2);
}