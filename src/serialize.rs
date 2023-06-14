use std::{collections::BTreeMap, fs::File};

use super::{Closure, Content, Document, Group, Id, Text};

type Deck<'a> = BTreeMap<(&'a str, &'a str), Vec<&'a str>>;

pub fn serialize(file: File, document: Document) {
    document.to_deck();
}

impl Document<'_> {
    fn to_deck(&self) -> Deck {
        let mut deck: Deck = BTreeMap::new();
        for rem in self.rems.iter() {
            let Id(id) = rem.id;
            let content = || rem.content.iter();
            for group in content().filter_map(|c| match c {
                Content::Closure(Closure {
                    group: Group(group),
                    text: _,
                }) => Some(group),
                _ => None,
            }) {
                if !deck.contains_key(&(id, group)) {
                    let vec: Vec<&str> = content()
                        .map(|content| match content {
                            Content::Closure(Closure {
                                group: Group(g),
                                text: _,
                            }) if g == group => "[...]",
                            Content::Text(Text(t))
                            | Content::Closure(Closure {
                                group: _,
                                text: Text(t),
                            }) => t,
                        })
                        .collect();
                    deck.insert((id, group), vec);
                }
            }
        }
        deck
    }
}
