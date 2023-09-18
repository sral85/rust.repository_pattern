use csv::Reader;
use serde::Deserialize;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Deserialize)]
struct Position {
    id: i32,
    value: i32,
}

trait Entity<K: Eq + PartialEq + Hash> {
    fn get_id(&self) -> K;
}

impl Entity<i32> for Position {
    fn get_id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug)]
struct Repository<T, K> {
    store: HashMap<K, T>,
}

impl<T: Entity<K>, K: Eq + PartialEq + Hash> Repository<T, K> {
    fn new() -> Self {
        Repository {
            store: HashMap::new(),
        }
    }

    fn add_object(&mut self, object: T) {
        self.store.insert(object.get_id(), object);
    }

    fn get_object(&self, key: K) -> Option<&T> {
        self.store.get(&key)
    }
}

fn main() {
    let mut repo = Repository::<Position, i32>::new();

    let mut reader = Reader::from_path("./position.csv").unwrap();
    for record in reader.deserialize() {
        match record{
            Ok(position) => repo.add_object(position),
            Err(err) => println!("An error occured: {:?}", err)
        }
    }

    println!("{:?}", &repo);
    println!("{:?}", &repo.get_object(1));
}
