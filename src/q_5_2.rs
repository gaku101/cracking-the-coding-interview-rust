pub fn binary_to_string(num: f64) -> String {
    if num <= 0.0 || num >= 1.0 {
        return "ERROR".to_string();
    }

    let mut num = num;
    let mut binary = String::from("0.");

    while num > 0.0 {
        if binary.len() - 2 >= 32 {
            return "ERROR".to_string();
        }

        let r = num * 2.0;
        if r >= 1.0 {
            binary.push('1');
            num = r - 1.0;
        } else {
            binary.push('0');
            num = r;
        }
    }
    binary
}
/*
計算量
Time Complexity: O(1)
→ 小数部の桁数は最大32桁と定数回の操作になるため。

Space Complexity: O(1)
→ 使用する変数や文字列の長さも最大で定数の長さに収まるため。


*/