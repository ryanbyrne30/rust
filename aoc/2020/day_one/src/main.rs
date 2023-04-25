use std::fs;
use crate::algs::sort;

pub mod algs;

const TARGET_SUM: i32 = 2020;

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

    let (a, b) = find_sum(&numbers);

    println!("{}, {} => {}", a, b, a * b);
}


fn find_sum(v_sorted: &Vec<i32>) -> (i32, i32) {
    let mut left: usize = 0;
    let mut right: usize = v_sorted.len() - 1;
    let mut sum: i32 = v_sorted[left] + v_sorted[right];

    while sum != TARGET_SUM && left < right {
        if sum < TARGET_SUM {
            left += 1
        } else if sum > TARGET_SUM {
            right -= 1
        }
        sum = v_sorted[left] + v_sorted[right]
    }

    if left >= right {
        panic!("No solution")
    }

    (v_sorted[left], v_sorted[right])
}