use std::fs;

const TREE: u8 = b'#';

pub fn solve() {
  let content = fs::read_to_string("src/days/day_03/input.txt").expect("Cannot read day 3 input file");
  let lines: Vec<&str> = content.split('\n').collect();
  let mut forest: Vec<&[u8]> = vec![];

  for line in lines {
    forest.push(line.as_bytes());
  }

  let sol_1: u32 = count_trees(&forest, (3, 1));

  println!("Day 3 - Problem 1: {}", sol_1);

  let slopes: [(usize, usize); 5] = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
  ];

  let mut sol_2: u128 = 1;
  for slope in slopes {
    sol_2 *= u128::from(count_trees(&forest, slope));
  }

  println!("Day 3 - Problem 2: {}", sol_2);
}

fn count_trees(forest: &Vec<&[u8]>, slope: (usize, usize)) -> u32 {
  let mut x: usize = slope.0;
  let mut y: usize = slope.1;
  let mut count: u32 = 0;
  let width: usize = forest[0].len();
  let height: usize = forest.len();

  while usize::from(y) < height {
    if forest[y][x] == TREE {
      count += 1;
    }

    x = (x + slope.0) % width;
    y = y + slope.1;
  }

  count
}