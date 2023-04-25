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
	part_one(&numbers);

	let sol2 = part_two(&numbers);
	println!("Solution to part 2 is: {}", sol2);
}

fn part_one(v_sorted: &Vec<i32>) {
	let (a, b) = find_sum(v_sorted);
	println!("Solution to part one is: Product of {} and {} is {}", a, b, a * b);
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

fn part_two(v_sorted: &Vec<i32>) -> i32 {
	let len = v_sorted.len();

	let mut i: usize = 0;
	while i < len {
		let mut j: usize = i + 1;
		while j < len {
			let mut k: usize = j + 1;
			while k < len {
				let sum = v_sorted[i] + v_sorted[j] + v_sorted[k];

				if sum == TARGET_SUM {
					return v_sorted[i] * v_sorted[j] * v_sorted[k];
				}

				k += 1;
			}

			j += 1;
		}

		i += 1;
	}

	panic!("No solution")
}