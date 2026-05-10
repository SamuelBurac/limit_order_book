use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Index as _;
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
const DEFAULT_MAX_USAGE_PERCENT: f64 = 0.75;

/** A numberline which is a list of lists
* Each entry in the list represents a price.
* Each price point has a list that acts as a queue.
* the first person to enter an order should have theirs
* fulfilled first.
*
*/
#[derive(Debug)]
pub struct NumberLine {
    head: Option<Weak<LineItemQueue>>,
    tail: Option<Weak<LineItemQueue>>,
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
            head: None,
            tail: None,
            items: vec![None; DEFAULT_INIT_CAPACITY],
            size: 0,
            capacity: DEFAULT_INIT_CAPACITY,
        }
    }

    pub fn add(&mut self, line_item: LineItem) {
        // check capacity used rate
        let used_percent = self.size as f64 / self.capacity as f64;
        if used_percent >= DEFAULT_MAX_USAGE_PERCENT {
            //TODO: resize map
        }

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
            //create new LIQ
            let liq = LineItemQueue::new(line_item);

            // Set the next and prev queue
            // for now just iterate until hit next and prev

            // forward iteration
            let mut fidx = index as usize + 1;
            while fidx < self.capacity {
                if let Some(next) = self.items.index(fidx) {
                    *liq.next_queue.borrow_mut() = Rc::downgrade(next);
                    break;
                }
                fidx += 1;
            }
            //     INFO: logging but not performant
            //     if (fidx == self.capacity) {
            //     println!("No next neighbor");
            //     }
            // reverse iteration finding prev
            let mut ridx = index as usize - 1;
            while ridx > 0 {
                if let Some(prev) = self.items.index(ridx) {
                    *liq.prev_queue.borrow_mut() = Rc::downgrade(prev);
                    break;
                }
                ridx -= 1;
            }
            if ridx == 0
                && let Some(prev) = self.items.index(ridx)
            {
                *liq.prev_queue.borrow_mut() = Rc::downgrade(prev);
            }

            // add new LIQ to the map at the index
            let liq_refc = Rc::new(liq);
            // check if numberline has head or tail first
            if self.head.is_none() {
                self.head = Some(Rc::downgrade(&liq_refc));
                self.tail = Some(Rc::downgrade(&liq_refc))
            }

            self.items[index as usize] = Some(liq_refc);
        }

        // if index is none create queue with LineItem
        // add next and previous queue refs to queue
        //
        // if index is not none increase size of items and
        // redo hash tableness.
    }

    fn resize() {
        // TODO: implement
    }
}

impl Default for NumberLine {
    fn default() -> Self {
        Self::new()
    }
}
