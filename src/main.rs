mod q_1_7;

fn main() {
    // 4x4の行列の例
    let mut matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];

    println!("回転前:");
    for row in &matrix {
        println!("{:?}", row);
    }

    // 行列を90度回転
    q_1_7::rotate_matrix(&mut matrix);

    println!("\n回転後:");
    for row in &matrix {
        println!("{:?}", row);
    }
}
