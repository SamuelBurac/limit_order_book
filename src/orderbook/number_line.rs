use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Index;
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

const DEFAULT_INIT_CAPACITY: usize = 100;

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
    capacity: usize,
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
    pub fn new(line_item: LineItem) -> LineItemQueue {
        let res = LineItemQueue {
            value: line_item.value,
            queue: RefCell::new(VecDeque::new()),
            next_queue: RefCell::new(Weak::new()),
            prev_queue: RefCell::new(Weak::new()),
        };
        res.queue.borrow_mut().push_front(line_item);
        res
    }

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
            items: vec![None; DEFAULT_INIT_CAPACITY],
            size: 0,
            capacity: DEFAULT_INIT_CAPACITY,
        }
    }

    pub fn add(&self, line_item: LineItem) {
        let index = line_item.value % self.capacity as u64;

        // check index
        if let Some(q) = self.items.index(index as usize) {
            // if the queue is for the same dollar value add the item to the queue
            if q.value == line_item.value {
                q.add(line_item);
            } else {
                // TODO: resize map
            }
        } else {
            let nouQ = LineItemQueue::new(line_item);
            //TODO: find next and previous queues

            // for now just iterate until hit next and prev
        }

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
