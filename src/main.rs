mod q_1_1;

fn main() {
    let test_str = "abcdef";
    println!(
        "Problem 1.1: Is \"{}\" unique? {}",
        test_str,
        q_1_1::is_unique(test_str)
    );
}
