use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct LineItem {
    id: u64,             // unique ID for an order
    value: u64,          // Dollar value
    count: RefCell<u64>, // Number of items at that price
}

#[derive(Debug)]
pub struct LineItemQueue {
    value: u64,
    queue: RefCell<VecDeque<LineItem>>,
    next_queue: RefCell<Weak<LineItemQueue>>,
    prev_queue: RefCell<Weak<LineItemQueue>>,
}

/** A numberline which is a list of lists
* Each entry in the list represents a price.
* Each price point has a list that acts as a queue.
* the first person to enter an order should have theirs
* fulfilled first.
*
*/
#[derive(Debug)]
pub struct NumberLine {
    max: u64,
    min: u64,
    items: Vec<Option<Rc<LineItemQueue>>>,
    size: usize,
}

impl LineItem {
    /** Reduce the count of items
     */
    pub fn reduce(&self, count: u64) {
        let mut curr_count = self.count.borrow_mut();
        *curr_count -= count;
    }
}

impl LineItemQueue {
    pub fn add(&self, line_item: LineItem) {
        self.queue.borrow_mut().push_back(line_item);
    }

    pub fn pop(&self) -> Option<LineItem> {
        self.queue.borrow_mut().pop_front()
    }
}

impl NumberLine {
    pub fn new() -> NumberLine {
        NumberLine {
            max: 0,
            min: 0,
            items: vec![None; 50],
            size: 0,
        }
    }

    pub fn add(&self, line_item: LineItem) {
        let index = line_item.value % self.items.len() as u64;
        // check index
        // if index is none create queue with LineItem
        // add next and previous queue refs to queue
        //
        // if index is not none increase size of items and
        // redo hash tableness.
    }
}

impl Default for NumberLine {
    fn default() -> Self {
        Self::new()
    }
}
