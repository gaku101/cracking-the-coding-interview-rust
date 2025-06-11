mod q_8_12;
use q_8_12::solve_n_queens;

fn main() {
    let n = 8;
    let solutions = solve_n_queens(n);
    println!("解の数: {}", solutions.len());

    for sol in &solutions {
        for &col in sol {
            let mut row_str = vec!['.'; n];
            row_str[col] = 'Q';
            println!("{}", row_str.iter().collect::<String>());
        }
        println!("---");
    }
}
/*
時間計算量： O(n!)（最悪ケース）
 実際は安全でない分岐は枝刈りされるため、n=8 で92通り

空間計算量： O(n)（1解あたり、再帰の深さn）
*/