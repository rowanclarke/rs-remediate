use super::{
    parser::{Content, Document, Rem},
    Deck,
};
use std::{collections::BTreeMap, rc::Rc};

impl Document {
    pub fn deck(&self) -> Deck {
        let mut deck = Deck::new();
        for rem in self.rems().iter() {
            rem.insert_into(&mut deck, 0);
        }
        deck
    }
}

impl Rem {
    pub fn insert_into(&self, deck: &mut Deck, depth: usize) {
        let mut card_buffer: BTreeMap<Rc<str>, Vec<Content>> = BTreeMap::new();
        for content in self.content().iter() {
            match content {
                Content::Closure((id, _), _) if id.clone() != self.id() => {
                    card_buffer.extend_at(id.clone(), content.clone())
                }
                _ => (),
            }
            for id in self.descendents() {
                card_buffer.extend_at(id, content.clone());
            }
        }
        for (id, vec) in card_buffer {
            deck.extend_at(id, (depth, vec));
        }
        for child in self.children().iter() {
            child.insert_into(deck, depth + 1);
        }
    }

    fn descendents(&self) -> Vec<Rc<str>> {
        let mut vec = Vec::new();
        self.add_descendents(&mut vec);
        vec
    }

    fn add_descendents(&self, vec: &mut Vec<Rc<str>>) {
        vec.push(self.id());
        for child in self.children().iter() {
            child.add_descendents(vec);
        }
    }
}

trait ExtendMap<K, T> {
    fn extend_at(&mut self, key: K, value: T);
}

impl<K, V, T> ExtendMap<K, T> for BTreeMap<K, V>
where
    V: Extend<T> + From<[T; 1]>,
    K: Ord,
{
    fn extend_at(&mut self, key: K, value: T) {
        if let Some(inner) = self.get_mut(&key) {
            inner.extend_one(value);
        } else {
            self.insert(key, [value].into());
        }
    }
}
