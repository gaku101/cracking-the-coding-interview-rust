mod q_1_4;

fn main() {
    let test_str = "tact coa";
    println!(
        "Is '{}' a palindrome permutation? {}",
        test_str,
        q_1_4::is_palindrome_permutation(test_str)
    );
}
