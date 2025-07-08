mod data_model;

use data_model::vector::Vector;
use data_model::item::Item;
use data_model::store::VectorStore;

use std::collections::HashMap;

fn main() {
    let data1: Vec<f32> = vec![0.1, 0.2, 0.3];
    let data2: Vec<f32> = vec![0.1, 0.2, 0.3];
    let data3: Vec<f32> = vec![0.1, 0.2, 0.3];

    let vector1: Vector = Vector::new(data1);
    let vector2: Vector = Vector::new(data2);
    let vector3: Vector = Vector::new(data3);

    let item1: Item = Item::new(String::from("abc123"), vector1, None);

    let item2: Item = Item::new(String::from("abc123"), vector2, None);

    let item3: Item = Item::new(String::from("abc456"), vector3, None);

    let mut store: VectorStore = VectorStore::new(HashMap::new());

    store.upsert(item1);

    let item_value = store.get("abc123");

    println!("{:#?}", item_value);

    store.delete("abc123");

    println!("{:#?}", store);

    store.upsert(item2);

    store.upsert(item3);

    println!("{:#?}", store);
}
