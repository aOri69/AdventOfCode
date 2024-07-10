use std::collections::VecDeque;

/// Simple queue implementation based
/// on `VecDeque`
///
/// Trait bound `PartialEq` needed if you
/// whant to use `contains()` method.
#[derive(Default, Debug)]
pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    /// Enqueue an element to the back of the queue
    pub fn enqueue(&mut self, element: T) {
        self.elements.push_back(element);
    }

    /// Dequeue an element from the front of the queue
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Get the size of the queue
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    // Peek at the front element of the queue without dequeuing it
    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }
}

impl<T: PartialEq> Queue<T> {
    /// Check if the queue contains a specific element
    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}
