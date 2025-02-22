mod q_3_6;

use q_3_6::{AnimalShelter, AnimalType};

fn main() {
    let mut shelter = AnimalShelter::new();

    // 動物を登録（到着順は順番にorderが増加）
    shelter.enqueue(AnimalType::Dog, "Buddy".to_string());
    shelter.enqueue(AnimalType::Cat, "Kitty".to_string());
    shelter.enqueue(AnimalType::Dog, "Max".to_string());
    shelter.enqueue(AnimalType::Cat, "Luna".to_string());

    // 登録された順番で最も古い動物を引き渡す
    if let Some(animal) = shelter.dequeue_any() {
        println!("dequeue_any: {:?}", animal);
    }

    // 犬の中で最も古い犬を引き渡す
    if let Some(dog) = shelter.dequeue_dog() {
        println!("dequeue_dog: {:?}", dog);
    }

    // 猫の中で最も古い猫を引き渡す
    if let Some(cat) = shelter.dequeue_cat() {
        println!("dequeue_cat: {:?}", cat);
    }
}
