use heap::min_heap::MaxHeap;

pub mod linked_lists;
pub mod heap;

fn main() {
    let values = vec![5, 3, 2, 5, 7, 9];
    let mut heap: MaxHeap<i32> = MaxHeap::new(10, 0);

    println!("Pushing values...");
    for value in values {
        heap.push(value);
    }
    heap.print_state();

    heap.update(2, 4);
    heap.update(3, 7);
    heap.print_state();


    println!("Popping values...");
    while heap.size > 0 {
        println!("Popped: {}", heap.pop());
    }
}
