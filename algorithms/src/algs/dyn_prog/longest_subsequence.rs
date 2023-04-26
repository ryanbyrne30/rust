pub fn solve(s1: &str, s2: &str) -> String {
  helper(s1.as_bytes(), s2.as_bytes())
}

fn helper(s1: &[u8], s2: &[u8]) -> String {
  let l1 = s1.len();
  let l2 = s2.len();

  if l1 == 0 || l2 == 0 {
    String::from("")
  } else if s1[l1 - 1] == s2[l2 - 1] {
    let mut result = helper(&s1[..l1-1], &s2[..l2-1]);
    result.push_str(&String::from(char::from(s1[l1 - 1])));
    result
  } else {
    let a = helper(&s1[..l1-1], s2);
    let b = helper(s1, &s2[..l2-1]);
    if a.len() > b.len() {
      a
    } else {
      b
    }
  }
}