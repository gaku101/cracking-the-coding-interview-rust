use std::collections::VecDeque;

#[derive(Debug)]
pub enum AnimalType {
    Dog,
    Cat,
}

#[derive(Debug)]
pub struct Animal {
    animal_type: AnimalType,
    #[allow(dead_code)]
    name: String,
    order: i32,
}

#[derive(Debug)]
pub struct AnimalShelter {
    dogs: VecDeque<Animal>,
    cats: VecDeque<Animal>,
    order: i32,
}

impl AnimalShelter {
    pub fn new() -> Self {
        Self {
            dogs: VecDeque::new(),
            cats: VecDeque::new(),
            order: 0,
        }
    }

    pub fn enqueue(&mut self, animal_type: AnimalType, name: String) {
        let animal = Animal {
            animal_type,
            name,
            order: self.order,
        };
        self.order += 1;
        match animal.animal_type {
            AnimalType::Dog => self.dogs.push_back(animal),
            AnimalType::Cat => self.cats.push_back(animal),
        }
    }

    pub fn dequeue_any(&mut self) -> Option<Animal> {
        match (self.dogs.front(), self.cats.front()) {
            (Some(dog), Some(cat)) => {
                if dog.order < cat.order {
                    self.dogs.pop_front()
                } else {
                    self.cats.pop_front()
                }
            }

            (Some(_), None) => self.dogs.pop_front(),
            (None, Some(_)) => self.cats.pop_front(),
            (None, None) => None,
        }
    }

    pub fn dequeue_dog(&mut self) -> Option<Animal> {
        self.dogs.pop_front()
    }

    pub fn dequeue_cat(&mut self) -> Option<Animal> {
        self.cats.pop_front()
    }
}
/*
時間計算量: 各操作は O(1)（VecDequeの先頭へのアクセス・追加・削除は定数時間）
空間計算量: O(n)（登録される動物の数 n に対してキューが必要）
*/

/*
VecDeque は、Rust の標準ライブラリ（std::collections）に含まれるデータ構造で、「両端キュー（Double-Ended Queue、Deque）」を実装しています。
通常の Vec は末尾への追加・削除が高速ですが、VecDeque は先頭および末尾の両方に対して高速な操作が可能です。

主な特徴
両端操作が効率的

push_front と pop_front による先頭での追加・削除
push_back と pop_back による末尾での追加・削除
これらの操作は、平均して O(1) の時間計算量で実行できます。
リングバッファとして実装

内部的には循環バッファ（リングバッファ）として管理され、メモリの再配置を必要最小限に抑えながら拡張されます。
用途

キューやデック（双方向キュー）としての利用に最適です。
両端からデータの追加・削除が必要な場合に Vec よりも効率的に動作します。
*/