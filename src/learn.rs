use crate::{Answers, Deck, REMEDY_DIR};
use bincode::deserialize_from;
use rand::prelude::*;
use std::{
    env,
    fs::{read_dir, File},
    io,
    io::prelude::Read,
};

pub fn learn() -> Result<(), ()> {
    let dir = &env::var(REMEDY_DIR).unwrap();
    let mut deck = Deck::<String>::new();
    let mut answers = Answers::<String>::new();
    for entry in read_dir(dir).unwrap().filter_map(Result::ok) {
        let (mut subdeck, mut subanswers) = deserialize_from::<_, (Deck<String>, Answers<String>)>(
            File::open(entry.path()).unwrap(),
        )
        .unwrap();
        deck.append(&mut subdeck);
        answers.append(&mut subanswers);
    }

    let mut rng = rand::thread_rng();
    let mut stdin = io::stdin();
    let mut pause = move || stdin.read(&mut [0u8]).unwrap();
    let keys: Vec<_> = deck.keys().collect();

    while let Some(key) = keys.choose(&mut rng) {
        println!("{}", deck.get(key).unwrap().concat());
        pause();
        println!("{}", answers.get(&key.0).unwrap().concat());
        pause();
    }

    Ok(())
}
