mod q_3_5;

use q_3_5::SortableStack;

fn main() {
    let mut stack = SortableStack::new();

    stack.push(3);
    stack.push(1);
    stack.push(4);
    stack.push(2);

    println!("Before sorting: {:?}", stack.input_stack);

    stack.sort();

    println!("After sorting: {:?}", stack.input_stack);
}
