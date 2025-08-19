// A `no_std` and no `alloc` library for more efficient array processing.
// Copyright (C) 2025  joker2770

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use core::mem::MaybeUninit;

/// Array-based circular queue implementation (optimized for Copy types)
pub struct ArrayQueue<T: Copy, const N: usize> {
    data: [T; N],       // Directly stores T type data
    head: usize,        // Queue head index
    len: usize,         // Current number of elements
    initialized: usize, // Number of initialized elements
}

impl<T: Copy, const N: usize> ArrayQueue<T, N> {
    /// Create a new empty queue
    /// Requires a default value to initialize the array
    pub fn new(default_value: T) -> Self {
        Self {
            data: [default_value; N],
            head: 0,
            len: 0,
            initialized: 0,
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

    /// Calculate the tail index
    fn tail(&self) -> usize {
        (self.head + self.len) % N
    }

    /// Try to add an element to the end of the queue
    /// Returns Err(element) if the queue is full
    pub fn enqueue(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }

        let tail = self.tail();

        // If the position is not initialized, increase the count
        if self.initialized < N {
            self.initialized += 1;
        }

        self.data[tail] = item;
        self.len += 1;
        Ok(())
    }

    /// Try to remove an element from the front of the queue
    /// Returns None if the queue is empty
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = self.data[self.head];
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(value)
    }

    /// Peek at the front element of the queue
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.data[self.head])
    }

    /// Peek at the front element of the queue as a mutable reference
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        Some(&mut self.data[self.head])
    }

    /// Clear the queue
    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }
}

/// Array-based circular queue implementation
pub struct AdvancedArrayQueue<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> AdvancedArrayQueue<T, N> {
    /// Create a new empty queue
    pub fn new() -> Self {
        // Safely initialize an array of N None values
        let data: [Option<T>; N] = {
            // Create an uninitialized array of MaybeUninit
            let mut data: [MaybeUninit<Option<T>>; N] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for elem in &mut data[..] {
                elem.write(None);
            }
            // Transmute to initialized array
            unsafe { core::mem::transmute_copy::<_, [Option<T>; N]>(&data) }
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

impl<T, const N: usize> Default for AdvancedArrayQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        // Use 0 as the default value
        let mut queue = ArrayQueue::<i32, 3>::new(0);

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

        let mut queue: AdvancedArrayQueue<i32, 3> = AdvancedArrayQueue::new();

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
        let mut queue = ArrayQueue::<i32, 3>::new(0);

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.enqueue(2), Ok(()));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.enqueue(3), Ok(()));
        assert_eq!(queue.enqueue(4), Ok(())); // Queue: [4, 2, 3]

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), None);

        let mut queue: AdvancedArrayQueue<i32, 3> = AdvancedArrayQueue::new();

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
        let mut queue = ArrayQueue::<i32, 3>::new(0);

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.enqueue(2), Ok(()));

        queue.clear();

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);

        let mut queue: AdvancedArrayQueue<i32, 3> = AdvancedArrayQueue::new();

        assert_eq!(queue.enqueue(1), Ok(()));
        assert_eq!(queue.enqueue(2), Ok(()));

        queue.clear();

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_peek_mut() {
        let mut queue = ArrayQueue::<i32, 3>::new(0);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();

        if let Some(value) = queue.peek_mut() {
            *value = 10;
        }

        assert_eq!(queue.peek(), Some(&10));
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(2));
    }

    // Test behavior of Copy types
    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_copy_type() {
        let default_point = Point { x: 0, y: 0 };
        let mut queue = ArrayQueue::<Point, 2>::new(default_point);

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };

        queue.enqueue(p1).unwrap();
        queue.enqueue(p2).unwrap();

        assert_eq!(queue.dequeue(), Some(p1));

        // The original value is still available (Copy trait)
        assert_eq!(p1.x, 1);
        assert_eq!(p1.y, 2);

        if let Some(p) = queue.peek_mut() {
            p.x = 10;
        }

        assert_eq!(queue.dequeue(), Some(Point { x: 10, y: 4 }));
    }

    #[test]
    fn test_initialization() {
        let mut queue = ArrayQueue::<i32, 3>::new(-1);

        // All elements are default values at the beginning
        assert_eq!(queue.data, [-1, -1, -1]);

        queue.enqueue(1).unwrap();
        assert_eq!(queue.data, [1, -1, -1]);

        queue.enqueue(2).unwrap();
        assert_eq!(queue.data, [1, 2, -1]);

        queue.dequeue();
        queue.enqueue(3).unwrap();
        assert_eq!(queue.data, [1, 2, 3]);

        queue.enqueue(4).unwrap();
        queue.enqueue(5).unwrap_err(); // Queue is full

        queue.dequeue();
        queue.enqueue(5).unwrap();
        assert_eq!(queue.data, [4, 5, 3]);
    }
}
