//! List: Singly Linked List
use list::List;

fn main() {
    let mut list = List::new();
    list.push_front("first");
    list.push_front("second");

    for (i, item) in list.iter().enumerate() {
        println!("{i}: {item}");
    }
    for (i, item) in list.iter_mut().enumerate() {
        println!("{i}: {item}");
    }
}
