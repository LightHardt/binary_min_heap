mod priority_queue;
use rand::Rng;

fn main() {
    let mut pq = priority_queue::PriorityQueue::new();
    
    // Generate random numbers for priority queue
    let mut rng = rand::thread_rng();

    // How many numbers to generate and add to queue
    // as well as dequeue
    let num = 50;

    for _ in 0..num {
        pq.enqueue(rng.gen_range(1..10000))
    }

    // Dequeue all numbers
    for _ in 0..num {
        match pq.dequeue() {
            Some(y) => print!("{} ",y),
            None => print!("None"),
        }
    }

    println!();
    
    // Should print Empty
    match pq.peek() {
        Some(x) => println!("{}",x),
        None => println!("Empty"),
    }

    pq.enqueue(3);
    pq.enqueue(2);
    pq.enqueue(1);

    // Should print 1 3 2
    pq.print();
    
}