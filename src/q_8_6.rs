pub fn hanoi(n: usize, from: char, aux: char, to: char) {
    if n == 0 {
        return;
    }
    hanoi(n - 1, from, to, aux);

    println!("Move disk {} from {} to {}", n, from, to);

    hanoi(n - 1, aux, from, to)
}
