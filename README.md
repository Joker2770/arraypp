# arraypp

[![Rust](https://github.com/Joker2770/arraypp/actions/workflows/rust.yml/badge.svg)](https://github.com/Joker2770/arraypp/actions/workflows/rust.yml)

A `no_std` and no `alloc` library for more efficient array processing, with functions such as tree, queue, stack, comparison,  filtering.

## Usage
Compare elements: 

```rust
    use arraypp::compare::ArrayExtrema;
    
    // Find both the minimum and maximum values and their indices at the same time
    let arr = [1.5, 3.2, 2.8, 4.7, 2.8];
    let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();
    
    println!("Min: {} at index {}", result.min.value, result.min.index);
    println!("Max: {} at index {}", result.max.value, result.max.index);
    
    // Find the minimum value and its index only
    let min = ArrayExtrema::min_with_index(&arr).unwrap();
    println!("Min: {} at index {}", min.value, min.index);
    
    // Find the maximum value and its index only
    let max = ArrayExtrema::max_with_index(&arr).unwrap();
    println!("Max: {} at index {}", max.value, max.index);
    
```

Queue options: 

```rust
    use arraypp::queue::ArrayQueue;
    
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
    
```

Stack options: 

```rust
    use arraypp::stack::ArrayStack;

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

```
