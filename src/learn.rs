use std::{env, fs::read_dir};

use super::{DeckOwned, REMEDY_DIR};

use bincode::deserialize_from;
use std::fs::File;

pub fn learn() -> Result<(), ()> {
    let dir = &env::var(REMEDY_DIR).unwrap();
    let mut deck = DeckOwned::new();
    for entry in read_dir(dir).unwrap().filter_map(Result::ok) {
        deck.append(
            &mut deserialize_from::<_, DeckOwned>(File::open(entry.path()).unwrap()).unwrap(),
        )
    }
    for card in deck.into_values() {
        println!("{}", card.concat());
    }
    Ok(())
}
