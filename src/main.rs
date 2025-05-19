mod q_8_4;
use q_8_4::get_subsets;

fn main() {
    let set = vec![1, 2, 3];
    let subsets = get_subsets(&set);

    for subset in subsets {
        println!("{:?}", subset);
    }
}
