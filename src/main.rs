mod q_3_4;

use q_3_4::QueueViaStacks;

fn main() {
    let mut queue = QueueViaStacks::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("{:?}", queue.dequeue()); // Some(1)
    println!("{:?}", queue.peek()); // Some(2)
    println!("{:?}", queue.dequeue()); // Some(2)
    println!("{:?}", queue.is_empty()); // false
    println!("{:?}", queue.dequeue()); // Some(3)
    println!("{:?}", queue.is_empty()); // true
}
