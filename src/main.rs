mod q_7_1;
use q_7_1::{Deck, Hand};

fn main() {
    // デッキを生成しシャッフル
    let mut deck = Deck::new();
    deck.shuffle();

    // プレイヤーの手札を作成
    let mut player_hand = Hand::new();

    // 2枚ドロー
    if let Some(card1) = deck.draw() {
        println!("Draw1: {:?} of {:?}", card1.rank, card1.suit);
        player_hand.add(card1);
    }
    if let Some(card2) = deck.draw() {
        println!("Draw2: {:?} of {:?}", card2.rank, card2.suit);
        player_hand.add(card2);
    }

    // 手札の得点を計算
    let score = player_hand.value();
    println!("Player hand value: {}", score);
}
