mod q_1_2;

fn main() {
    let s1 = "abc";
    let s2 = "bca";
    println!("Are '{}' and '{}' permutations? {}", s1, s2, q_1_2::check_permutation2(s1, s2)); // true

    let s1 = "abc";
    let s2 = "abcd";
    println!("Are '{}' and '{}' permutations? {}", s1, s2, q_1_2::check_permutation2(s1, s2)); // false
}
