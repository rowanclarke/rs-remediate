use super::parser::*;
use super::CardId;
use super::Deck;
use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::{collections::BTreeMap, rc::Rc};

type Rems = BTreeMap<Rc<str>, Vec<Content>>;

impl Document {
    pub fn deck(&self) -> Deck {
        let rems = self.rems();
        let mut deck = BTreeMap::new();
        for (id, contents) in rems.iter() {
            for content in contents {
                if let Ok((location, group, _)) = content.closure() {
                    deck.insert(
                        CardId {
                            id: id.clone(),
                            location,
                            group,
                        },
                        vec![],
                    );
                }
            }
        }
        for (id, segments) in deck.iter_mut() {
            for content in rems.get(&id.id).unwrap() {
                segments.push(match content.closure() {
                    Ok((l, g, text)) if (&l, &g) == (&id.location, &id.group) => {
                        Segment::Hidden(text)
                    }
                    Ok((_, _, text)) | Err(text) => Segment::Visible(text),
                })
            }
        }
        deck
    }

    fn rems(&self) -> Rems {
        let mut rems = Rems::new();
        for rem in self.rems_iter() {
            rem.insert_into(&mut rems, &mut vec![]);
        }
        rems
    }
}

impl Rem {
    fn insert_into(&self, rems: &mut Rems, parents: &mut Vec<Rc<str>>) -> Vec<(Rc<str>, usize)> {
        let mut subtree = vec![];
        parents.push(self.id());

        for child in self.children_iter() {
            subtree.extend(
                child
                    .insert_into(rems, parents)
                    .into_iter()
                    .map(|(id, n)| (id, n + 1)),
            );
        }
        subtree.push((self.id(), 0));

        for content in self.content_iter() {
            match content.closure() {
                Ok((location, group, text)) if location > 0 => {
                    let parent = parents[parents.len() - location - 1].clone();
                    rems.extend_at(parent, Content::to_closure(0, group.clone(), text.clone()));
                }
                _ => (),
            }
        }

        for (id, offset) in subtree.iter() {
            for content in self.content_iter() {
                match content.closure() {
                    Ok((location, group, text)) => {
                        rems.extend_at(
                            id.clone(),
                            Content::to_closure(location + offset, group, text),
                        );
                    }
                    Err(text) => rems.extend_at(id.clone(), Content::to_text(text)),
                }
            }
        }

        parents.pop();
        subtree
    }
}

#[derive(Debug, Serialize, Deserialize, Archive)]
#[archive(check_bytes)]
pub enum Segment {
    Visible(Rc<str>),
    Hidden(Rc<str>),
}

pub enum DisplayCardStatus {
    Show,
    Hide,
}

pub struct DisplayCard {
    segments: Vec<Segment>,
    status: DisplayCardStatus,
}

impl DisplayCard {
    pub fn new(segments: Vec<Segment>) -> Self {
        Self {
            segments,
            status: DisplayCardStatus::Hide,
        }
    }

    pub fn show(&mut self) {
        self.status = DisplayCardStatus::Show;
    }

    pub fn hide(&mut self) {
        self.status = DisplayCardStatus::Hide;
    }
}

impl Display for DisplayCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in self.segments.iter() {
            match (segment, &self.status) {
                (Segment::Visible(text), _) => write!(f, "{}", text)?,
                (Segment::Hidden(_), DisplayCardStatus::Hide) => write!(f, "[...]")?,
                (Segment::Hidden(text), DisplayCardStatus::Show) => write!(f, "[{}]", text)?,
            }
        }
        Ok(())
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
