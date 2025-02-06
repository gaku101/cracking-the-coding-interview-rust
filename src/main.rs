mod q_1_9;

fn main() {
    let s1 = "waterbottle";
    let s2 = "erbottlewat";

    if q_1_9::is_rotation(s1, s2) {
        println!("'{}' は '{}' の回転です。", s2, s1);
    } else {
        println!("'{}' は '{}' の回転ではありません。", s2, s1);
    }
}
