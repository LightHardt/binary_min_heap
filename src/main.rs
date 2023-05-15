mod priority_queue;
use rand::Rng;

fn main() {
    let mut pq = priority_queue::PriorityQueue::new();
    
    // Generate random numbers for priority queue
    let mut rng = rand::thread_rng();

    // How many numbers to generate and add to queue
    // as well as dequeue
    let num = 10;

    for _ in 0..num {
        pq.enqueue(rng.gen_range(1..1000))
    }

    print!("Array View: ");
    pq.print();

    print!("Sorted View: ");
    // Dequeue all numbers
    for _ in 0..num {
        match pq.dequeue() {
            Some(y) => print!("{} ",y),
            None => print!("None"),
        }
    }
    println!();

    // Use just to remove warning
    pq.peek();
    
}