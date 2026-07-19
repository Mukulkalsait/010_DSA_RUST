use crate::linked_list::a2_generic_singli_list::GSinglyLinkedList;

// mod chapters;
mod linked_list;

/// Y:
/// NO MOVE use REFERENCING:
/// | Operation      | Use      |
/// | -------------- | -------- |
/// | read value     | `&T`     |
/// | modify value   | `&mut T` |
/// | take ownership | `T`      |
///
/// G: Move => Expencive.
///    Refernce => Fast + Efficient + Safe.
///
fn main() {
    // chapters::run();
    // linked_list::singly_list::main();

    let mut list: GSinglyLinkedList<i32> = linked_list::a2_generic_singli_list::GSinglyLinkedList::new();

    println!("-----------------------");
    list.push_front(23);
    list.push_front(834);
    list.push_front(9983);
    list.push_front(459);
    list.push_front(0);
    list.push_front(12);

    let x = list.len();
    println!("{:?} is the length", x);

    list.print_list();
    list.peek_front();
    let x = list.pop_front();
    println!("{:?} is the popped element", x);
    println!("-------------------");
    let x = list.pop_front();
    println!("{:?} is the popped element", x);
    println!("-------------------");
    let x = list.peek_front();
    println!("{:?} is being peeked", x);
    let x = list.peek_mut();
    println!("{:?} is being peeked mutably", x);
    let x = list.len();
    println!("{:?} is the length", x);
}
