use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Ace,
    Num(u8),
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    // ブラックジャックでの値を返す（Ace は 1 または 11 を呼び出し側で調整）
    pub fn blackjack_value(&self) -> u8 {
        match self.rank {
            Rank::Ace => 1,
            Rank::Num(n) => n,
            Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            cards.push(Card {
                suit,
                rank: Rank::Ace,
            });
            for n in 2..=10 {
                cards.push(Card {
                    suit,
                    rank: Rank::Num(n),
                });
            }
            cards.push(Card {
                suit,
                rank: Rank::Jack,
            });
            cards.push(Card {
                suit,
                rank: Rank::Queen,
            });
            cards.push(Card {
                suit,
                rank: Rank::King,
            });
        }
        Deck { cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&self) -> u8 {
        let mut sum = 0;
        let mut ace_count = 0;
        for &c in &self.cards {
            if let Rank::Ace = c.rank {
                ace_count += 1;
            }
            sum += c.blackjack_value();
        }
        for _ in 0..ace_count {
            if sum + 10 <= 21 {
                sum += 10
            }
        }
        sum
    }
}
