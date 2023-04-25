use std::fs;
use crate::algs::sort;

pub mod algs;

fn main() {
    let file_path = String::from("src/input.txt");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");
    let inputs: Vec<&str> = contents.split('\n').collect::<Vec<&str>>();

    let mut numbers: Vec<i32> = vec![];

    for input in inputs {
        let n: i32 = input.trim().parse().expect("Not a number");
        numbers.push(n)
    }

    sort::qsort(&mut numbers);

    for number in numbers {
        println!("{}", number);
    }
}
