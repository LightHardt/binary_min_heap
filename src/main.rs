mod priority_queue;
fn main() {
    let mut pq = priority_queue::PriorityQueue::new();

    pq.enqueue(78);
    pq.enqueue(79);
    pq.enqueue(90);
    pq.enqueue(64);
    pq.enqueue(78);
    pq.enqueue(65);
    pq.enqueue(78);
    
    // pattern matching to handle Option<i32> return
    match pq.peek() {
        Some(x) => println!("{}",x),
        None => println!("Empty"),
    }

    pq.dequeue();
    pq.dequeue();
    
    match pq.peek() {
        Some(x) => println!("{}",x),
        None => println!("Empty"),
    }

    pq.print();
    
}