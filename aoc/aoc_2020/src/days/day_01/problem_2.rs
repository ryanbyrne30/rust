
// Find 3 numbers in numbers that sum to target and return product
pub fn solve(numbers: &Vec<i32>, target: i32) -> i32 {
  let len = numbers.len();

	let mut i: usize = 0;
	while i < len {
		let mut j: usize = i + 1;
		while j < len {
			let mut k: usize = j + 1;
			while k < len {
				let sum = numbers[i] + numbers[j] + numbers[k];

				if sum == target {
					return numbers[i] * numbers[j] * numbers[k];
				}

				k += 1;
			}

			j += 1;
		}

		i += 1;
	}

	panic!("No solution")
}