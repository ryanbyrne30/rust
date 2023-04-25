pub fn is_valid_password(password: &str, letter: u8, a: usize, b: usize) -> bool {
  let bytes = password.as_bytes();

  let c1 = match bytes.get(a - 1) {
    None => return false,
    Some(c) => *c
  };

  let c2 = match bytes.get(b - 1) {
    None => return false,
    Some(c) => *c
  };

  (c1 == letter || c2 == letter) && c1 != c2
}