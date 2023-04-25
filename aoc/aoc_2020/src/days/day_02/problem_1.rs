pub fn is_valid_password(password: &str, letter: u8, a: usize, b: usize) -> bool {
  let count = usize::from(count_letters(password, letter));
  count >= a && count <= b
}

fn count_letters(password: &str, target: u8) -> u8 {
  let mut count: u8 = 0;
  let bytes: std::str::Bytes = password.bytes();

  for char in bytes {
    if char == target {
      count += 1;
    }
  }
  count
}