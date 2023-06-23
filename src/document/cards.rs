use super::parser::*;
use super::RemGroupCard;
use rkyv::{Archive, Deserialize, Serialize};
use std::{collections::BTreeMap, rc::Rc};

type RemContent = BTreeMap<Rc<str>, Vec<Content>>;

impl Document {
    pub fn card_map(&self) -> RemGroupCard {
        let content_map = self.content_map();
        let mut card_map = BTreeMap::new();
        for (id, contents) in content_map.iter() {
            for content in contents {
                if let Ok((location, group, _)) = content.closure() {
                    card_map.insert((id.clone(), location, group), vec![]);
                }
            }
        }
        for ((id, location, group), segments) in card_map.iter_mut() {
            for content in content_map.get(id).unwrap() {
                segments.push(match content.closure() {
                    Ok((l, g, text)) if &l == location && &g == group => Segment::Hidden(text),
                    Ok((_, _, text)) | Err(text) => Segment::Visible(text),
                })
            }
        }
        card_map
    }

    fn content_map(&self) -> RemContent {
        let mut content_map = RemContent::new();
        for rem in self.rems() {
            rem.insert_into(&mut content_map, &mut vec![]);
        }
        content_map
    }
}

impl Rem {
    fn insert_into(
        &self,
        map: &mut BTreeMap<Rc<str>, Vec<Content>>,
        parents: &mut Vec<Rc<str>>,
    ) -> Vec<(Rc<str>, usize)> {
        let mut subtree = vec![];
        parents.push(self.id());

        for child in self.children() {
            subtree.extend(
                child
                    .insert_into(map, parents)
                    .into_iter()
                    .map(|(id, n)| (id, n + 1)),
            );
        }
        subtree.push((self.id(), 0));

        for content in self.content() {
            match content.closure() {
                Ok((location, group, text)) if location > 0 => {
                    let parent = parents[parents.len() - location - 1].clone();
                    map.extend_at(parent, Content::to_closure(0, group.clone(), text.clone()));
                }
                _ => (),
            }
        }

        for (id, offset) in subtree.iter() {
            for content in self.content() {
                match content.closure() {
                    Ok((location, group, text)) => {
                        map.extend_at(
                            id.clone(),
                            Content::to_closure(location + offset, group, text),
                        );
                    }
                    Err(text) => map.extend_at(id.clone(), Content::to_text(text)),
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
