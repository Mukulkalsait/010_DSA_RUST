pub struct GenericSinglyLinkedList<T> {
    pub head: Option<Box<GenericNode<T>>>,
}

#[derive(Debug)]
pub struct GenericNode<T> {
    pub data: T,
    pub next: Option<Box<GenericNode<T>>>,
}
