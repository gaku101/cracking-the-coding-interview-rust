mod q_8_7;

use q_8_7::permutations;

fn main() {
    let input = "rust";
    for p in permutations(input) {
        println!("{}", p);
    }
}
