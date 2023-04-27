pub fn solve(s1: &str, s2: &str) -> String {
  helper(s1.as_bytes(), s2.as_bytes(), s1.len(), s2.len())
}

fn helper(s1: &[u8], s2: &[u8], m: usize, n: usize) -> String {
  if m == 0 {
    match std::str::from_utf8(&s2[..n]) {
      Ok(v) => v.to_string(),
      Err(e) => panic!("Invalid u8 string. {}", e)
    }
  } else if n == 0 {
    match std::str::from_utf8(&s1[..m]) {
      Ok(v) => v.to_string(),
      Err(e) => panic!("Invalid u8 string. {}", e)
    }
  } else if s1[m - 1] == s2[n - 1] {
    let mut result = helper(s1, s2, m - 1, n - 1);
    result.push_str(&String::from(char::from(s1[m - 1])));
    result
  } else {
    let mut a = helper(s1, s2, m - 1, n);
    let mut b = helper(s1, s2, m, n - 1);

    a.push_str(&String::from(char::from(s1[m - 1])));
    b.push_str(&String::from(char::from(s2[n - 1])));

    if a.len() < b.len() {
      a
    } else {
      b
    }
  }
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn shortest_supersequence_works() {
    let result = solve("ABCBDAB", "BDCABA");
    let solutions = [
      "ABCBDCABA",
      "ABDCABDAB",
      "ABDCBDABA"
    ];

    let mut is_valid = false;

    for solution in solutions {
      if result == solution {
        is_valid = true;
        break;
      }
    }

    assert!(is_valid, "Received: {}", result);
  }
}