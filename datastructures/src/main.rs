use linked_lists::problem_01::{create_list_from_array, List};

pub mod linked_lists;

fn main() {
    let values = vec![4, 3, 2, 5, 7, 9];
    let mut list: List<i32> = List::new();

    println!("List before...");
    list.summary();

    create_list_from_array(&values, &mut list);

    println!("List after...");
    list.summary();

    let mid = list.middle();

    match mid {
        Some(n) => {
            println!("Middle element is: {}", n.data)
        },
        None => panic!("No middle element")
    }

}
