use std::collections::VecDeque;
use std::fmt::Debug;

pub struct LimitedQueue<T> {
    queue: VecDeque<T>,
    limit: usize,
}

impl<T: Eq + Debug> LimitedQueue<T> {
    pub fn new(limit: usize) -> Self {
        LimitedQueue {
            queue: VecDeque::new(),
            limit,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.queue.len() < self.limit {
            self.queue.push_back(item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn contains(&mut self, item: T) -> bool {
        return self.queue.contains(&item);
    }

    pub fn len(&self) -> usize {
        return self.queue.len();

    }

    // TODO rust idiomatic
    pub fn to_string(&self) -> String {
        let queue = &self.queue;
        return format!("{queue:?}");
    }
}
