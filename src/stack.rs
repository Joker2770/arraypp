use core::mem::MaybeUninit;
use core::ptr;

pub struct ArrayStack<T: Copy, const N: usize> {
    data: [T; N], // Fixed-size array to store elements
    top: usize,   // Stack top pointer (points to the next free position)
}

impl<T: Copy, const N: usize> ArrayStack<T, N> {
    /// Create a new stack (requires initializing the array with a default value)
    pub const fn new(default_value: T) -> Self {
        ArrayStack {
            data: [default_value; N],
            top: 0,
        }
    }

    /// Push operation
    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            Err("Stack full")
        } else {
            self.data[self.top] = item;
            self.top += 1;
            Ok(())
        }
    }

    /// Pop operation
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            Err("Stack empty")
        } else {
            self.top -= 1;
            Ok(self.data[self.top])
        }
    }

    /// Peek at the top element
    pub fn peek(&self) -> Result<&T, &'static str> {
        if self.is_empty() {
            Err("Stack empty")
        } else {
            Ok(&self.data[self.top - 1])
        }
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// Check if the stack is full
    pub fn is_full(&self) -> bool {
        self.top == N
    }

    /// Current number of elements in the stack
    pub fn len(&self) -> usize {
        self.top
    }
}

pub struct AdvancedArrayStack<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    top: usize,
}

impl<T, const N: usize> AdvancedArrayStack<T, N> {
    /// Create a new empty stack
    pub const fn new() -> Self {
        // Safety: MaybeUninit does not require initialization
        AdvancedArrayStack {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            top: 0,
        }
    }

    /// Push operation
    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Stack full");
        }

        // Write data
        self.data[self.top].write(item);
        self.top += 1;
        Ok(())
    }

    /// Pop operation
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            return Err("Stack empty");
        }

        self.top -= 1;
        // Safety: We know this position is initialized
        Ok(unsafe { self.data[self.top].assume_init_read() })
    }

    /// Peek at the top element
    pub fn peek(&self) -> Result<&T, &'static str> {
        if self.is_empty() {
            return Err("Stack empty");
        }

        // Safety: We know this position is initialized
        Ok(unsafe { &*self.data[self.top - 1].as_ptr() })
    }

    /// Peek at the top element as a mutable reference
    pub fn peek_mut(&mut self) -> Result<&mut T, &'static str> {
        if self.is_empty() {
            return Err("Stack empty");
        }

        // Safety: We know this position is initialized
        Ok(unsafe { &mut *self.data[self.top - 1].as_mut_ptr() })
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// Check if the stack is full
    pub fn is_full(&self) -> bool {
        self.top == N
    }

    /// Current number of elements in the stack
    pub fn len(&self) -> usize {
        self.top
    }

    /// Clear the stack and return an iterator
    pub fn drain(&mut self) -> Drain<'_, T, N> {
        Drain { stack: self }
    }
}

impl<T, const N: usize> Default for AdvancedArrayStack<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Drop handler to process initialized elements
impl<T, const N: usize> Drop for AdvancedArrayStack<T, N> {
    fn drop(&mut self) {
        // Only drop initialized elements
        for i in 0..self.top {
            unsafe {
                ptr::drop_in_place(self.data[i].as_mut_ptr());
            }
        }
    }
}

/// Iterator for draining the stack
pub struct Drain<'a, T, const N: usize> {
    stack: &'a mut AdvancedArrayStack<T, N>,
}

impl<'a, T, const N: usize> Iterator for Drain<'a, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.pop().unwrap()) // We know the stack is not empty
        }
    }
}

impl<'a, T, const N: usize> Drop for Drain<'a, T, N> {
    fn drop(&mut self) {
        // Consume remaining elements
        while self.next().is_some() {}
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut stack: ArrayStack<i32, 3> = ArrayStack::new(0);

        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);

        assert!(stack.push(1).is_ok());
        assert_eq!(stack.peek(), Ok(&1));
        assert_eq!(stack.len(), 1);

        assert!(stack.push(2).is_ok());
        assert_eq!(stack.pop(), Ok(2));

        assert!(stack.push(3).is_ok());
        assert!(stack.push(4).is_ok());
        assert!(stack.is_full());

        assert_eq!(stack.push(5), Err("Stack full"));

        assert_eq!(stack.pop(), Ok(4));
        assert_eq!(stack.pop(), Ok(3));
        assert_eq!(stack.peek(), Ok(&1));
        assert_eq!(stack.pop(), Ok(1));
        assert_eq!(stack.pop(), Err("Stack empty"));
    }
}
