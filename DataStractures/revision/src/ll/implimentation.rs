#[derive(serde::Serialize)]
struct ErrResp {
    error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum GenericSinglyLinkedListErrors {
    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("resources not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal server error")]
    Internal,
}

use crate::ll::blueprint::{GenericNode, GenericSinglyLinkedList};
impl<T> GenericSinglyLinkedList<T> {
    /// new fn
    pub fn new() -> Self {
        GenericSinglyLinkedList { head: None }
    }

    /// push front
    pub fn push_front(&mut self, val: T) {
        let new_node = Box::new(GenericNode { data: val, next: self.head.take() });
        self.head = Some(new_node);
    }

    pub fn push_last(&mut self, val: T) {
        let new_node = Box::new(GenericNode { data: val, next: None });

        let mut crn = &mut self.head;
        while let Some(x_node) = crn {
            crn = &mut x_node.next;
        }
        *crn = Some(new_node); // here dereferenced CRN is already head of last node...
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

    pub fn length(&self) -> usize {
        let mut cnr = &self.head;
        let mut length: usize = 0;
        while let Some(x) = cnr {
            length += 1;
            cnr = &x.next;
        }
        length
    }

    pub fn is_empty(&self) -> bool {
        self.head.as_ref().is_none()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            x.data
        })
    }
}

impl<T: std::fmt::Debug> GenericSinglyLinkedList<T> {
    pub fn print_all(&self) {
        print!("<|Head|-");

        let mut crn = &self.head;
        while let Some(x) = crn {
            print!("->{:?}|-", &x.data);
            crn = &x.next;
        }
        print!("-|END]");
    }

    pub fn insert_at(&mut self, val: T, index: usize) -> Result<(), GenericSinglyLinkedListErrors> {
        match index {
            0 => {
                self.push_front(val);
                Ok(())
            }
            _ => {
                let mut crn = &mut self.head;

                for _ in 0..(index - 1) {
                    match crn {
                        Some(node) => {
                            crn = &mut node.next;
                        }
                        None => return Err(GenericSinglyLinkedListErrors::NotFound),
                    }
                }
                if let Some(node) = crn {
                    let new_node = Box::new(GenericNode { data: val, next: node.next.take() });
                    node.next = Some(new_node);
                    Ok(())
                } else {
                    Err(GenericSinglyLinkedListErrors::NotFound)
                }
            }
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Result<T, GenericSinglyLinkedListErrors> {
        match index {
            0 => self.pop_front().ok_or(GenericSinglyLinkedListErrors::NotFound),
            _ => {
                let mut crn = &mut self.head;
                for _ in 0..(index - 1) {
                    match crn {
                        Some(node) => crn = &mut node.next,
                        None => return Err(GenericSinglyLinkedListErrors::NotFound),
                    }
                }
                if let Some(node) = crn {
                    let mut target = node.next.take().ok_or(GenericSinglyLinkedListErrors::NotFound)?;
                    // IMP: ok_or(...) => returns Resualt<T,E>
                    // vs
                    //  ok_or(...)? => return T
                    node.next = target.next.take();
                    Ok(target.data)
                } else {
                    Err(GenericSinglyLinkedListErrors::NotFound)
                }
            }
        }
    }

    pub fn reverse_list(&mut self) {
        let mut prev_node = None;
        let mut current_node = self.head.take(); // Y: 1 currentNode

        while let Some(mut current_x) = current_node {
            let temp_next = current_x.next.take(); // Y: temp_next = current+1
            current_x.next = prev_node; // Y: Current>Next => Previous
            prev_node = Some(current_x); // Y: Previous => Current
            current_node = temp_next; // Y: Current => Temp_nxt
        }
        self.head = prev_node;
    }

    /// in this fun, we aree only taking reference so &self taken
    /// but in return we need data so Resualt<&T, Err> here &T needed.
    pub fn find_middle(&self) -> Result<&T, GenericSinglyLinkedListErrors> {
        if self.head.is_none() {
            Err(GenericSinglyLinkedListErrors::NotFound)
        } else {
            let mut slow = self.head.as_ref();
            let mut fast = self.head.as_ref();

            while let Some(f_node) = fast {
                if let Some(next) = f_node.next.as_ref() {
                    fast = next.next.as_ref(); // fast 2 STEP
                    if let Some(s_node) = slow {
                        slow = s_node.next.as_ref();
                    }
                } else {
                    break;
                }
            }
            slow.map(|node| &node.data).ok_or(GenericSinglyLinkedListErrors::NotFound)
        }
    }
}
