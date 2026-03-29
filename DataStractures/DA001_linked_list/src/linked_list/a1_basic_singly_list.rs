/// Option<Box<Node>>:
///  - Heap + Recursive Size Known + Some(Box<Node>),None
///  * this is one box and multiple boxes are managed by SinglySlinkedList Struct
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

/// Wrapper
/// ---------------------------
/// SinglySlinkedList
///   🔻
///  Head -> N1 -> N2 -> N3
/// ---------------------------
pub struct SinglylinkedList {
    head: Option<Box<Node>>,
}

impl SinglylinkedList {
    /// info
    fn new_ssl() -> Self {
        SinglylinkedList { head: None }
    }
}

impl SinglylinkedList {
    /// info
    fn push_ssl(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node)
    }
}

impl SinglylinkedList {
    /// info
    fn print_ssl(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            println!("{}", node.val);
            current = &node.next;
        }
    }
}

impl SinglylinkedList {
    fn length_ssl(&self) {
        let mut current = &self.head;
        let mut length: u32 = 0;

        while let Some(node) = current {
            length += 1;
            current = &node.next;
        }
        println!("Total Length of the SinglyLinkedList is : {}", length);
    }
}

// impl SinglylinkedList {
//     fn pop_ssl(&mut self, val: i32, x: Node) {
//         let mut current = &self.head;
//     }
// }
//
pub fn main() {
    let mut list = SinglylinkedList::new_ssl();

    list.push_ssl(12);
    list.push_ssl(23);
    list.push_ssl(43);
    list.push_ssl(43);
    list.push_ssl(43);
    list.push_ssl(43);
    list.print_ssl();
    list.length_ssl();
}
