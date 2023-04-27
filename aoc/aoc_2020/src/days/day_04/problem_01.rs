use lazy_static::lazy_static;
use regex::Regex;

pub fn solve(ids: &Vec<&str>) {
  let mut count: i32 = 0;

  for id in ids {
    if is_valid_id(id) {
      count += 1;
    }
  }

  println!("Solution for Day 04 - Problem 01: {} valid ids", count);
}

fn is_valid_id(id: &str) -> bool {
  lazy_static! {
    static ref RE_BIRTH: Regex = Regex::new("byr").unwrap();
    static ref RE_ISSUE: Regex = Regex::new("iyr").unwrap();
    static ref RE_EXP: Regex = Regex::new("eyr").unwrap();
    static ref RE_HEIGHT: Regex = Regex::new("hgt").unwrap();
    static ref RE_HAIR: Regex = Regex::new("hcl").unwrap();
    static ref RE_EYE: Regex = Regex::new("ecl").unwrap();
    static ref RE_PID: Regex = Regex::new("pid").unwrap();
    static ref RE_CID: Regex = Regex::new("cid").unwrap();
  }

  RE_BIRTH.is_match(id)
  && RE_ISSUE.is_match(id)
  && RE_EXP.is_match(id)
  && RE_HEIGHT.is_match(id)
  && RE_HAIR.is_match(id)
  && RE_EYE.is_match(id)
  && RE_PID.is_match(id)
}