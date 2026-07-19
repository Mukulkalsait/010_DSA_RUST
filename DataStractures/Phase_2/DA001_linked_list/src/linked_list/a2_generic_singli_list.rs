/// # Blue-Print: GNode
/// Option<Box<Node>>:
/// - Heap + Recursive Size Known + Some(Box<Node>),None
/// * this is one box and multiple boxes are managed by SinglySlinkedList Struct
/// > data | next
/// -| analogy > Trains Boggie
#[derive(Debug)]
pub struct GNode<T> {
    pub data: T,
    pub next: Option<Box<GNode<T>>>,
}

/// # Blue-Print: GSinglyLinkedList
///
/// ---------------------------
/// SinglySlinkedList
///   🔻
///  Head -> N1 -> N2 -> N3
/// ---------------------------
/// > head
/// with Head => whole list => 1 variable  IMP:
/// therefore Head |> starting point...
/// -| analogy > Engine of Train... to controll.
/// ### Insid the impl of GSinglyLinkedList:
///   - Self will automacitally become => GSinglyLinkedList.
pub struct GSinglyLinkedList<T> {
    pub head: Option<Box<GNode<T>>>,
}

impl<T> GSinglyLinkedList<T> {
    ///
    /// - Create new list with help of GSinglyLinkedList::<head>
    /// - Empty list = no nodes → so head points to nothing → so (head = None)
    /// ## IMP : here Self (capital Self) mean type itself.
    ///      fn x()->Self {GSinglyLinkedList<T>} // data returned to fill Blue-print.
    ///
    /// ## IMP: whereas self means object its holding:
    ///     pub fn push_front(&mut self){...}
    ///     // self here is the data inside Self
    pub fn new() -> Self {
        GSinglyLinkedList { head: None }
    }

    /// # G: Push the new element to 1st element... (FRONT)
    ///
    /// ## Struct is a blueprint & Impl is actual Object creation: hence ->
    /// - we have to define everyting
    /// - create new Box -> Box::new()
    /// - the box of type GNode =>
    /// - where values are like {data:__,next__,}
    /// # Return
    /// - finally return new_node header into self.head
    /// > box->new->Gnode => put the location in self.head
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(GNode {
            data: value,
            next: self.head.take(), // we give the head of 1st to next of new
        });
        self.head = Some(new_node); // we set the new to to 1st with self.head
    }

    /// # G: remove the 1st element and return its value| make 2nd element 1st.
    ///
    /// - take head of 1st element - store in x_node
    /// - use self.head(next nodes position) => map to x_node.next
    /// - and returned the x_node.data => which is ther in fn Option<T>
    /// > X_node
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|x_node| {
            self.head = x_node.next;
            x_node.data
        })
    }

    /// as_ref => is equal to asking "&" reference
    /// hence
    /// ```rust
    /// self.head.as_ref().map(|x_node| &x_node.data)
    /// ```
    /// in this line we use as_ref and then &x_node
    /// .map() mean
    ///     if Some => map that
    ///     if None => keep None
    ///
    /// so basically map is taking data from
    /// self.head.as_ref => and giving to x_node
    /// and we return &x_node.data
    ///
    /// Y: .map(|x| x*3) - is the Interator part fn
    /// read the Notes to know more about iterator lifecycle
    /// data -> iterator -> filter -> mapper -> adapter -> consumer -> return
    /// .iter() /.iter_mut() / .into_iter()
    ///
    /// ##### Case: 1
    ///     head = Some(Box(Node { data: 10 }))
    ///       as_ref() → Some(&Box(Node))
    ///       map() → Some(&10)
    /// ##### Case: 2
    ///     head = None
    ///       as_ref() → None
    ///       map() → None
    ///       
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|x_node| &x_node.data)
        // if we do &self.head.....
        // this will give us &Options<Box<Node>>
        // what we needed is Optios<&Box<Node>>
        // so we removed the self.head then used .as_ref() fun.
        // difference
        // &Options -> reference to whole options.
        // OPtions<&Box> -> Options providing Reference
        // so if we have
        // Some(&Box<Node { data: 10 }>)
        // Return :
        // |x_node| &x_node.data => 10
        // final resualt Optiosn<&T> which is Return Type.
    }

    /// Peak_front but mutable...
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| &mut x.data)
    }

    /// # G: check the length of the list
    ///
    /// in loop inside
    /// here we are doing:
    ///  - while -> loop on condiction
    ///
    /// this line->
    /// ```rust
    /// While let Some(x) = current{...}
    /// //  is same as
    /// loop {
    /// match current {
    ///    Some(node) => { ... }
    ///     None => break,
    /// }
    /// ```
    ///  - let Some(<CONDITION>)
    ///  - Some(x_node) = current :
    ///     * this mean current comes to x_node,
    ///     * if the current dont give amyting to x_node = Stop.
    ///  - use of X_node is inside block therefore x_node
    ///  - x_node name to prove name can be anyting even x only.
    /// > count=0| current=self.head <iterator> | return count (usize)
    pub fn len(&self) -> usize {
        let mut length: usize = 0;
        let mut current = &self.head;

        while let Some(x_node) = current {
            length += 1;
            current = &x_node.next;
        }
        length
    }

    /// if hte list is empty then its head must contain None
    /// so we used is_none() fn directly on the reference of self.head
    /// > &self.head |=| bool
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// we used match case for checking where to:
    /// if at 0 => self.push_front()
    /// *_ => &mut current_runntin_node => index times transverse.
    ///  we return current_running_node = (current_runntin_node.as_mut.unwrap.NEXT.take)
    /// create mutable new_node set data: val and next = None
    /// now new_node.next = current_runntin_node(which is acutlaly CNR.next 🔺)
    /// and current_runntin_node.as_mut.unwrapednext = Some(new_node)
    ///
    /// this help undersnand funciton a log.
    pub fn insert_at(&mut self, index: usize, val: T) {
        match index {
            0 => {
                self.push_front(val);
            }
            _ => {
                let mut current_running_node = &mut self.head;
                for _ in 0..(index - 1) {
                    current_running_node = &mut current_running_node.as_mut().unwrap().next;
                }
                let mut new_node = Box::new(GNode {
                    data: val,
                    next: None,
                    // current_running_node ... is already &mut .mut().unwraped()... and all
                });
                new_node.next = current_running_node.take();
                current_running_node.as_mut().unwrap().next = Some(new_node);
            }
        }
    }

    /// # remove at =>
    ///  ## we do match case if index = 0
    ///  ```rust
    ///  self.pop_front()
    ///  else
    ///   let crn = &mut self.head
    ///   loopon index.
    ///     new crn =>
    ///     let target - crn.next
    ///     crn.next = target.next
    ///      return target.data
    /// | &mut self, index |-| Option<T>
    /// ```
    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        match index {
            0 => self.pop_front(),
            _ => {
                let mut current = &mut self.head;

                for _ in 0..(index - 1) {
                    current = &mut current.as_mut()?.next;
                }
                let mut target = current.as_mut()?.next.take()?;
                current.as_mut()?.next = target.next.take();
                Some(target.data)
            }
        }
    }

    /// now thi one is 🎃
    /// let prefv = None
    /// let crn - &ref self .head .take() // consumption function into_iter() Style
    /// now
    ///  while let Some(x) = crn  {
    ///  let temp_next =  x.next.take();
    ///  x.next= prev
    ///  crn = x.next()
    ///  prev = some(x).
    ///
    ///  }
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut x_node) = current {
            let temperory_next = x_node.next.take(); // save next
            x_node.next = prev; // reverse link
            prev = Some(x_node); // move prev forward
            current = temperory_next; // move current forward
        }
        self.head = prev;
    }
}

impl<T: std::fmt::Debug> GSinglyLinkedList<T> {
    /// # G: Look at the first element without removing it.
    pub fn print_list(&self) {
        print!("List: ");
        let mut current = &self.head;
        while let Some(x_node) = current {
            print!("{:?} -> ", x_node.data);
            current = &x_node.next;
        }
        println!("None")
    }

    pub fn hard_reset_list(&mut self) {
        self.head = None
    }
}
