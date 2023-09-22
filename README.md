# binary_min_heap
Priority queue implemented with binary min heap in rust 🦀

## Usage
Clone repo to a desired location and add to project with `cargo add --path "path to folder"`

## Types
This was meant to be used with integers and unsigned integers (i.e. i16, i32, u16, u32) the implementation for the generic is tightly bound and may not work with the type you want

## Example
```
use binary_min_heap::PriorityQueue;
use rand::Rng;

fn main() {
    let mut queue = PriorityQueue::<i16>::new();
    let mut rng = rand::thread_rng();
    let num_of_elements = 10000;

    for _ in 0..num_of_elements {
        queue.enqueue(rng.gen_range(0..i16::MAX));
    }

    for _ in 0..queue.len() {
        println!("Value: {}", match queue.dequeue() {
            Some(x) => x,
            None => i16::MIN,
        });
    }
}
```
