use std::fmt::Display;

pub struct MaxHeap<T: Display + Copy + PartialOrd> {
  data: Vec<T>,
  max_size: usize,
  pub size: usize,
}

impl<T: Display + Copy + PartialOrd> MaxHeap<T> {
  pub fn new(size: usize, init: T) -> Self {
    MaxHeap { data: vec![init; size], max_size: size, size: 0 }
  }

  fn parent_index(&self, index: usize) -> usize {
    if index == 0 {
      0
    } else if index >= self.size {
      panic!("Index out of bounds. Heap size is {}. Index was {}.", self.size, index)
    } else {
      (index - 1) / 2
    }
  }

  fn left_child_index(&self, index: usize) -> Option<usize> {
    let child_index = index * 2 + 1;

    if index >= self.size {
      panic!("Index out of bounds. Heap size is {}. Index was {}.", self.size, index)
    } else if child_index >= self.size {
      None
    } else {
      Some(child_index)
    }
  }

  fn right_child_index(&self, index: usize) -> Option<usize> {
    let child_index = index * 2 + 2;

    if index >= self.size {
      panic!("Index out of bounds. Heap size is {}. Index was {}.", self.size, index)
    } else if child_index >= self.size {
      None
    } else {
      Some(child_index)
    }
  }

  fn swap(&mut self, i: usize, j: usize) {
    let temp: T = self.data[i];
    self.data[i] = self.data[j];
    self.data[j] = temp;
  }

  fn bubble_up(&mut self, index: usize) {
    let mut i = index;
    while i > 0 && self.data[self.parent_index(i)] < self.data[i] {
      self.swap(self.parent_index(i), i);
      i = self.parent_index(i);
    }
  }

  fn bubble_down(&mut self, index: usize) {
    if index > self.size {
      panic!("Index out of bounds. Heap size is {}. Index was {}.", self.size, index);
    }

    let l = self.left_child_index(index);
    let r = self.right_child_index(index);
    let mut largest = index;

    match l {
      Some(li) => {
        if self.data[li] > self.data[largest] {
          largest = li
        }
      },
      None => {}
    }

    match r {
      Some(ri) => {
        if self.data[ri] > self.data[largest] {
          largest = ri
        }
      },
      None => {}
    }

    // if heap is not satisfied bubble down
    if largest != index {
      self.swap(largest, index);
      self.bubble_down(largest);
    }
  }

  pub fn push(&mut self, value: T) {
    if self.size >= self.max_size {
      panic!("Heap is full. Max size is {}. Size is {}.", self.max_size, self.size);
    }

    if self.size == 0 {
      self.data[0] = value;
      self.size += 1;
      return;
    }

    // initially insert element at end of heap
    self.size += 1;
    let index = self.size - 1;
    self.data[index] = value;

    // bubble element up until heap is satisfied
    self.bubble_up(index);
  }

  pub fn pop(&mut self) -> T {
    if self.size <= 0 {
      panic!("Heap is empty");
    }
    if self.size == 1 {
      self.size -= 1;
      self.data[0]
    } else {
      let value = self.data[0];
      self.data[0] = self.data[self.size - 1];
      self.size -= 1;
      self.bubble_down(0);
      value
    }
  }

  pub fn update(&mut self, index: usize, value: T) {
    let current_value = self.data[index];
    self.data[index] = value;

    if value > current_value {
      self.bubble_up(index);
    } else if value < current_value {
      self.bubble_down(index);
    }
  }

  pub fn print_state(&self) {
    for i in 0..self.size {
      print!("{} ", self.data[i]);
    }
    println!();
  }
}