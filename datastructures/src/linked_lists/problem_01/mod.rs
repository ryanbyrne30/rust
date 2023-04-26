// https://www.geeksforgeeks.org/write-a-c-function-to-print-the-middle-of-the-linked-list/

use std::fmt::Display;

pub struct Node<T: Display + Copy> {
  pub data: T,
  pub next: Option<Box<Node<T>>>,
}

pub struct List<T: Display + Copy> {
  pub head: Option<Box<Node<T>>>,
  pub length: usize,
}

impl<T: Display + Copy> List<T> {
  pub fn new() -> Self {
    List { head: None, length: 0 }
  }

  pub fn push(&mut self, value: T) {
    let new_node = Box::new(Node {
      data: value,
      next: self.head.take()
    });
    self.head = Some(new_node);
    self.length += 1;
  }

  pub fn middle(&self) -> &Option<Box<Node<T>>> {
    let mi = self.length / 2;
    let mut node = &self.head;
    for _i in 0..mi {
      match node {
        Some(n) => {
          node = &n.next;
        },
        None => panic!("Index out of bounds. Could not find middle of list.")
      }
    }
    node
  }

  pub fn summary(&self) {
    let mut cur_node = &self.head;
    let mut count: usize = 0;

    while !cur_node.is_none() {
      match cur_node {
        Some(node) => {
          print!("{}=>{}, ", count, node.data);
          cur_node = &node.next
        },
        None => break
      }
      count += 1;
    }
    println!();
  }
}

// pub fn find_middle(list: &List) {
//   let middle: usize = (list.length + 1) / 2;
//   println!("Middle: {}", middle);
// }

pub fn create_list_from_array<T: Display + Copy>(values: &Vec<T>, list: &mut List<T>) {
  for value in values {
    list.push(*value);
  }
}
