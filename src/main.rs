mod q_1_8;

fn main() {
    let mut matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 0, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 0],
    ];

    println!("更新前:");
    for row in &matrix {
        println!("{:?}", row);
    }

    q_1_8::zero_matrix_constant(&mut matrix);

    println!("\n更新後:");
    for row in &matrix {
        println!("{:?}", row);
    }
}
