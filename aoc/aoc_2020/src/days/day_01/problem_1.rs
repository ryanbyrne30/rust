// Find 2 numbers in numbers that sum to target and return product
pub fn solve(numbers: &Vec<i32>, target: i32) -> i32 {
  let mut left: usize = 0;
  let mut right: usize = numbers.len() - 1;

  while left < right {
    let a: i32 = numbers[left];
    let b: i32 = numbers[right];

    if a + b == target {
      return a * b;
    } else if a + b > target {
      right -= 1;
    } else {
      left += 1;
    }
  }

  panic!("No solution")
}