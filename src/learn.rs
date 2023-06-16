use std::{env, fs::read_dir};

use crate::{Answers, Deck, REMEDY_DIR};

use bincode::deserialize_from;
use std::fs::File;

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
    for card in deck.into_values() {
        println!("{}", card.concat());
    }
    for answer in answers.into_values() {
        println!("{}", answer.concat());
    }
    Ok(())
}
