mod q_8_9;
use q_8_9::generate_parens;

fn main() {
    let n = 3;
    let patterns = generate_parens(n);

    for p in patterns {
        println!("{}", p);
    }
}