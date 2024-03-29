use std::fmt::Display;

// Priority Queue implemented with binary min heap
pub struct PriorityQueue<T> {
    heap: Vec<T>,
}
impl<T: Ord + Eq + Display + Copy + Clone> PriorityQueue<T> {

    // Generate priority queue with empty heap
    pub fn new() -> Self {
        let heap = Vec::<T>::new();
        let pq = PriorityQueue { heap };
        pq
    }
    // Check to see if heap is empty
    pub fn is_empty(&self) -> bool {
        if self.heap.is_empty() {
            return true;
        }
        return false;
    }

    // Returns min element in heap without removing it
    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        return Some(self.heap[0]);
    }

    // Print heap values
    pub fn print(&self) {
        for x in &self.heap {
            print!("{} ",x);
        }
        println!();
    }

    // Add element to heap and maintain heap by percolating value up if needed
    pub fn enqueue(&mut self, val: T) {
        if self.is_empty() {
            self.heap.push(val);
            return;
        }

        // Add value to back of vector
        self.heap.push(val);
        // Keep track of index to later percolate up as needed
        let mut child_index = self.heap.len() - 1;
        
        // Percolate up
        while child_index != 0 {
            // Check if even index (right node/leaf)
            if child_index % 2 == 0 {
                // Get parent index with given formula and store value
                let parent_index = (child_index / 2) - 1;
                let parent = self.heap[parent_index];

                // If child is less then parent swap positions
                if self.heap[child_index] < parent {
                    self.heap[parent_index] = self.heap[child_index];
                    self.heap[child_index] = parent;
                    child_index = parent_index;
                }
                // If child is not less then break loop done swapping
                else {
                    break;
                }
            }
            // Odd index so left node/leaf
            else {
                // Get parent index and value with formula
                let parent_index = child_index / 2;
                let parent = self.heap[parent_index];

                if self.heap[child_index] < parent {
                    self.heap[parent_index] = self.heap[child_index];
                    self.heap[child_index] = parent;
                    child_index = parent_index;
                }
                else {
                    break;
                }
            }
        } 
    }

    // Remove min value and maintain min heap status
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        // Get root value as will return it at end
        let root = Some(self.heap[0]);
        // Overrite first element with last
        self.heap[0] = self.heap[self.heap.len() - 1];
        self.heap.pop();

        // Variables used for loop to keep track of indexes
        let mut index = 0;
        loop {
            // Grab children of parent with given formulas
            let left_node = (index * 2) + 1;
            let right_node = (index * 2) + 2;

            // Check to see if both nodes exist
            if left_node < self.heap.len() && right_node < self.heap.len() {
                // Determine smallest node between left and right child for potential swap
                // with parent
                if self.heap[left_node] < self.heap[right_node] {
                    // Check if node is smaller then parent and if so swap
                    if self.heap[left_node] < self.heap[index] {
                        let temp = self.heap[index];
                        self.heap[index] = self.heap[left_node];
                        self.heap[left_node] = temp;
                        index = left_node;
                    }
                    // If the smaller of two nodes isnt smaller break loop
                    else {
                        break;
                    }
                }
                else {
                    if self.heap[right_node] < self.heap[index] {
                        let temp = self.heap[index];
                        self.heap[index] = self.heap[right_node];
                        self.heap[right_node] = temp;
                        index = right_node;
                    }
                    else {
                        break;
                    }
                }
            }
            // If both nodes dont exist try left leaf
            else if left_node < self.heap.len() {
                if self.heap[left_node] < self.heap[index] {
                    let temp = self.heap[index];
                    self.heap[index] = self.heap[left_node];
                    self.heap[left_node] = temp;
                    index = left_node;
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }

        return root;
    }

    pub fn len(&self) -> usize{
        return self.heap.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let mut pq = PriorityQueue::new();

        pq.enqueue(5);
        assert_eq!(Some(5),pq.peek());
        
        pq.dequeue();
        assert_eq!(None,pq.peek());
    }

    #[test]
    fn test_is_empty() {
        let mut pq = PriorityQueue::new();

        assert!(pq.is_empty());
        
        pq.enqueue(5);
        assert!(!pq.is_empty());

        pq.dequeue();
        assert!(pq.is_empty());
    }

    #[test]
    fn test_enqueue() {
        let mut pq = PriorityQueue::new();

        pq.enqueue(1);
        assert_eq!(Some(1),pq.peek());

        pq.enqueue(-1);
        assert_eq!(Some(-1),pq.peek());

        pq.enqueue(5);
        assert_eq!(Some(-1),pq.peek());

        pq.enqueue(-2);
        assert_eq!(Some(-2),pq.peek());
    }

    #[test]
    fn test_dequeue() {
        let mut pq = PriorityQueue::new();

        pq.enqueue(5);
        pq.enqueue(2);
        pq.enqueue(10);
        pq.enqueue(7);

        assert_eq!(Some(2),pq.dequeue());
        assert_eq!(Some(5),pq.dequeue());
        assert_eq!(Some(7),pq.dequeue());
        assert_eq!(Some(10),pq.dequeue());
    }
}