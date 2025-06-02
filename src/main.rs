mod q_8_10;
use q_8_10::{Color, paint_fill};

fn main() {
    let mut screen = vec![
        vec![Color::Red, Color::Red, Color::Green],
        vec![Color::Red, Color::Red, Color::Green],
        vec![Color::Green, Color::Green, Color::Green],
    ];

    paint_fill(&mut screen, 0, 0, Color::Blue);

    for row in screen {
        println!("{:?}", row);
    }

    // 出力（Red領域がBlueに塗られる）:
    // [Blue, Blue, Green]
    // [Blue, Blue, Green]
    // [Green, Green, Green]
}