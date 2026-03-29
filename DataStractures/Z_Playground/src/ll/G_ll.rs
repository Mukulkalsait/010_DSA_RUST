#[derive(Debug)]
pub struct GNode<T> {
    pub data: T,
    pub next: Option<Box<GNode<T>>>,
}
pub struct GSLL<T> {
    pub head: Option<Box<GNode<T>>>,
}

impl<T> GSLL<T> {
    pub fn new() -> Self {
        GSLL { head: None }
    }
}
