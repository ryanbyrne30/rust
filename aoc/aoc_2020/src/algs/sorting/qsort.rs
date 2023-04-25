pub fn qsort(v: &mut Vec<i32>) {
  qsort_helper(v, 0, v.len() - 1)
}

fn qsort_helper(v: &mut Vec<i32>, low: usize, high: usize) {
  if low < high {
    let pi = partition(v, low, high);
    qsort_helper(v, low, pi - 1);
    qsort_helper(v, pi + 1, high);
  }
}

fn partition(v: &mut Vec<i32>, low: usize, high: usize) -> usize {
  let pivot: i32 = v[high].clone();
  let mut cursor: usize = low;
  let mut i: usize = low;

  while cursor <= high {
    let cursor_val = v[cursor].clone();
    if cursor_val < pivot {
      swap(v, i, cursor);
      i += 1;
    }
    cursor += 1;
  }

  swap(v, i, high);
  i
}

fn swap(v: &mut Vec<i32>, i: usize, j: usize) {
  let temp = v[i];
  v[i] = v[j];
  v[j] = temp;
}


#[cfg(test)]
mod tests {
    use crate::algs::sorting::qsort::{partition, qsort};


  #[test]
  fn partition_works() {
    let mut v: Vec<i32> = vec![5, 4, 8, 3, 4, 9, 7];
    let expected = [5, 4, 3, 4, 7, 9, 8];
    let pi = partition(&mut v, 0, 6);
    assert_eq!(pi, 4);

    for (i, &value) in v.iter().enumerate() {
      assert_eq!(expected[i], value);
    }
  }

  #[test]
  fn qsort_works() {
    let mut v: Vec<i32> = vec![5, 4, 8, 3, 4, 9, 7];
    let expected = [3, 4, 4, 5, 7, 8, 9];
    qsort(&mut v);

    for (i, &value) in v.iter().enumerate() {
      assert_eq!(expected[i], value);
    }
  }
}