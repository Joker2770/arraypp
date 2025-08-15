/// Array-based circular queue implementation
pub struct ArrayQueue<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> ArrayQueue<T, N> {
    /// Create a new empty queue
    pub fn new() -> Self {
        // Initialize an array of N None values
        let data = {
            // We need a temporary uninitialized array
            #[allow(unused_unsafe)]
            let mut data: [Option<T>; N] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };

            // Initialize all elements to None
            for elem in &mut data[..] {
                *elem = None;
            }
            data
        };

        Self {
            data,
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if the queue is full
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Return the current number of elements in the queue
    pub fn len(&self) -> usize {
        self.len
    }

    /// Try to add an element to the end of the queue
    /// Returns Err(element) if the queue is full
    pub fn enqueue(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }

        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % N;
        self.len += 1;

        Ok(())
    }

    /// Try to remove an element from the front of the queue
    /// Returns None if the queue is empty
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.data[self.head].take();
        self.head = (self.head + 1) % N;
        self.len -= 1;

        item
    }

    /// Peek at the front element of the queue without removing it
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.data[self.head].as_ref()
    }

    /// Peek at the front element of the queue as a mutable reference without removing it
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        self.data[self.head].as_mut()
    }

    /// Clear the queue
    pub fn clear(&mut self) {
        while self.dequeue().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut queue: ArrayQueue<i32, 3> = ArrayQueue::new();

        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.enqueue(2), Ok(()));
        assert_eq!(queue.enqueue(3), Ok(()));

        assert!(queue.is_full());
        assert_eq!(queue.len(), 3);

        assert_eq!(queue.enqueue(4), Err(4)); // Queue is full

        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));

        assert_eq!(queue.len(), 2);

        assert_eq!(queue.enqueue(4), Ok(())); // Now there is space

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_wrap_around() {
        let mut queue: ArrayQueue<i32, 3> = ArrayQueue::new();

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.enqueue(2), Ok(()));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.enqueue(3), Ok(()));
        assert_eq!(queue.enqueue(4), Ok(())); // Now the queue should be [4, 2, 3]

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_clear() {
        let mut queue: ArrayQueue<i32, 3> = ArrayQueue::new();

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.enqueue(2), Ok(()));

        queue.clear();

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }
}
