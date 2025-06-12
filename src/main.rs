mod q_8_13;
use q_8_13::{BoxDim, max_stack_height};

fn main() {
    let boxes = vec![
        BoxDim { width: 4, depth: 4, height: 4 },
        BoxDim { width: 3, depth: 3, height: 3 },
        BoxDim { width: 2, depth: 2, height: 2 },
        BoxDim { width: 1, depth: 1, height: 1 },
    ];

    let max_height = max_stack_height(&boxes);
    println!("最大の積み上げ高さ: {}", max_height); // 出力: 10
}