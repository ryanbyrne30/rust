use lazy_static::lazy_static;
use regex::Regex;

pub fn solve(ids: &Vec<&str>) {
  let mut count: i32 = 0;

  for id in ids {
    if is_valid_id(id.to_string()) {
      count += 1;
    }
  }

  println!("Solution for Day 04 - Problem 02: {} valid ids", count);
}

fn is_valid_id(id: String) -> bool {
  is_valid_birth(&id)
  && is_valid_height(&id)
  && is_valid_issue(&id)
  && is_valid_exp(&id)
  && is_valid_hair(&id)
  && is_valid_eye(&id)
  && is_valid_pid(&id)
}

fn parse_info_from_string(rgx: &Regex, string: &String) -> Option<String> {
  match rgx.captures(string) {
    Some(caps) => {
      Some(caps[0].to_string())
    },
    None => {
      None
    }
  }
}

fn is_valid_birth(id: &String) -> bool {
  lazy_static! {
    static ref RE_BIRTH: Regex = Regex::new(r"byr\:\d{4}").unwrap();
    static ref RE_YEAR: Regex = Regex::new(r"\d{4}").unwrap();
  }

  match parse_info_from_string(&RE_BIRTH, id) {
    None => {
      false
    },
    Some(v) => {
      match parse_info_from_string(&RE_YEAR, &v) {
        None => {
          false
        },
        Some(yr) => {
          let year: u32 = yr.trim().parse().expect(&format!("Could not parse year as u32: {}", yr));
          year >= 1920 && year <= 2002
        }
      }
    }
  }
}

fn is_valid_height(id: &String) -> bool {
  lazy_static! {
    static ref RE_HEIGHT: Regex = Regex::new(r"hgt\:\d+(cm|in)").unwrap();
    static ref RE_NUM: Regex = Regex::new(r"\d+").unwrap();
    static ref RE_UNIT: Regex = Regex::new(r"(cm|in)").unwrap();
  }

  match parse_info_from_string(&RE_HEIGHT, id) {
    None => { false },
    Some(string) => {
      match (parse_info_from_string(&RE_NUM, &string), parse_info_from_string(&RE_UNIT, &string)) {
        (Some(n), Some(u)) => {
          let num: u32 = n.trim().parse().expect(&format!("Could not parse string as u32: {}", n));
          if u == "in" {
            num >= 59 && num <= 76
          } else if u == "cm" {
            num >= 150 && num <= 193
          } else {
            false
          }
        },
        _ => { false }
      }
    }
  }
}

fn is_valid_issue(id: &String) -> bool {
  lazy_static! {
    static ref RE_FULL: Regex = Regex::new(r"iyr\:\d{4}").unwrap();
    static ref RE_YEAR: Regex = Regex::new(r"\d{4}").unwrap();
  }

  match parse_info_from_string(&RE_FULL, id) {
    None => {false},
    Some(string) => {
      match parse_info_from_string(&RE_YEAR, &string) {
        None => {false},
        Some(yr) => {
          let year: u32 = yr.trim().parse().expect(&format!("Could not parse issue year: {}", yr));
          year >= 2010 && year <= 2020
        }
      }
    }
  }
}

fn is_valid_exp(id: &String) -> bool {
  lazy_static! {
    static ref RE_FULL: Regex = Regex::new(r"eyr\:\d{4}").unwrap();
    static ref RE_YEAR: Regex = Regex::new(r"\d{4}").unwrap();
  }

  match parse_info_from_string(&RE_FULL, id) {
    None => {false},
    Some(string) => {
      match parse_info_from_string(&RE_YEAR, &string) {
        None => {false},
        Some(yr) => {
          let year: u32 = yr.trim().parse().expect(&format!("Could not parse issue year: {}", yr));
          year >= 2020 && year <= 2030
        }
      }
    }
  }
}

fn is_valid_hair(id: &String) -> bool {
  lazy_static! {
    static ref RE_HAIR: Regex = Regex::new(r"hcl\:\#([0-9abcdef]){6}").unwrap();
  }
  parse_info_from_string(&RE_HAIR, id).is_some()
}

fn is_valid_eye(id: &String) -> bool {
  lazy_static! {
    static ref RE_EYE: Regex = Regex::new(r"ecl\:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
  }
  parse_info_from_string(&RE_EYE, id).is_some()
}

fn is_valid_pid(id: &String) -> bool {
  lazy_static! {
    static ref RE_PID: Regex = Regex::new(r"pid\:[0-9]{9}(\s|$)").unwrap();
  }
  parse_info_from_string(&RE_PID, id).is_some()
}


#[cfg(test)]
mod tests {
  use crate::days::day_04::problem_02::{is_valid_id, is_valid_birth, is_valid_height, is_valid_hair, is_valid_eye, is_valid_pid};

  #[test]
  fn is_invalid_passport() {
    let passports = [
      "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
      "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
      "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
      "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
    ];

    for passport in passports {
      assert!(!is_valid_id(passport.to_string()));
    }
  }

  #[test]
  fn is_valid_passport() {
    let passports = [
      "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
      "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
      "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
      "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
    ];

    for passport in passports {
      assert!(is_valid_id(passport.to_string()));
    }
  }

  #[test]
  fn validates_birth() {
    let valid_inputs = [
      "byr:2002",
    ];

    let invalid_inputs = [
      "byr:2003"
    ];

    for input in valid_inputs {
      assert!(is_valid_birth(&input.to_string()), "Was not valid: {}", input);
    }

    for input in invalid_inputs {
      assert!(!is_valid_birth(&input.to_string()), "Did not register as invalid: {}", input);
    }
  }

  #[test]
  fn validates_height() {
    let valid_inputs = [
      "hgt:60in",
      "hgt:190cm",
    ];

    let invalid_inputs = [
      "hgt:190in",
      "hgt:190",
    ];

    for input in valid_inputs {
      assert!(is_valid_height(&input.to_string()), "Was not valid: {}", input);
    }

    for input in invalid_inputs {
      assert!(!is_valid_height(&input.to_string()), "Did not register as invalid: {}", input);
    }
  }

  #[test]
  fn validates_hair() {
    let valid_inputs = [
      "hcl:#123abc",
    ];

    let invalid_inputs = [
      "hcl:#123abz",
      "hcl:123abc"
    ];

    for input in valid_inputs {
      assert!(is_valid_hair(&input.to_string()), "Was not valid: {}", input);
    }

    for input in invalid_inputs {
      assert!(!is_valid_hair(&input.to_string()), "Did not register as invalid: {}", input);
    }
  }

  #[test]
  fn validates_eye() {
    let valid_inputs = [
      "ecl:brn",
    ];

    let invalid_inputs = [
      "ecl:wat",
    ];

    for input in valid_inputs {
      assert!(is_valid_eye(&input.to_string()), "Was not valid: {}", input);
    }

    for input in invalid_inputs {
      assert!(!is_valid_eye(&input.to_string()), "Did not register as invalid: {}", input);
    }
  }

  #[test]
  fn validates_pid() {
    let valid_inputs = [
      "pid:000000001",
    ];

    let invalid_inputs = [
      "pid:0123456789",
    ];

    for input in valid_inputs {
      assert!(is_valid_pid(&input.to_string()), "Was not valid: {}", input);
    }

    for input in invalid_inputs {
      assert!(!is_valid_pid(&input.to_string()), "Did not register as invalid: {}", input);
    }
  }
}