use super::{
    document::{Content, Rem},
    Cards,
};
use std::{collections::BTreeMap, rc::Rc};

impl Rem {
    pub fn insert_into(&self, cards: &mut Cards, depth: usize) {
        let mut card_buffer: BTreeMap<Rc<str>, Vec<Content>> = BTreeMap::new();
        let content = self.content();
        let mut iter = content.iter().peekable();
        while let Some(content) = iter.next() {
            match content {
                Content::Closure((id, _), _) if id.clone() != self.id() => {
                    card_buffer.extend_at(id.clone(), content.clone());
                    if iter.peek().is_some() {
                        card_buffer.extend_at(id.clone(), Content::Text(" ... ".into()));
                    }
                }
                _ => (),
            }
            for id in self.descendants() {
                card_buffer.extend_at(id, content.clone());
            }
        }
        for (id, vec) in card_buffer {
            cards.extend_at(id, (depth, vec));
        }
        for child in self.children().iter() {
            child.insert_into(cards, depth + 1);
        }
    }

    fn descendants(&self) -> Vec<Rc<str>> {
        let mut vec = Vec::new();
        self.add_descendants(&mut vec);
        vec
    }

    fn add_descendants(&self, vec: &mut Vec<Rc<str>>) {
        vec.push(self.id());
        for child in self.children().iter() {
            child.add_descendants(vec);
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
            inner.extend([value]);
        } else {
            self.insert(key, [value].into());
        }
    }
}
