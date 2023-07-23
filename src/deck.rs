mod cards;
pub mod document;
use crate::{
    archive::Cast,
    loc,
    workspace::{AsComponents, IntoComponents, Workspace},
    DIR,
};
use rkyv::{ser::serializers::AllocSerializer, ser::Serializer, Archive, Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    rc::Rc,
};

pub use document::{Content, Group};
use rkyv::de::deserializers::SharedDeserializeMap;

use self::document::Document;

const PATH: &str = "documents";

type Cards = BTreeMap<Rc<str>, Card>;

type DeckSerializer = AllocSerializer<1024>;

#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive_attr(derive(Debug))]
pub struct Deck {
    cards: Cards,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: Cards::new(),
        }
    }

    pub fn cards(&self) -> &Cards {
        &self.cards
    }

    pub fn all<W: Workspace>(
        workspace: &W,
    ) -> impl Iterator<Item = (Self, Rc<[W::Component]>)> + '_ {
        workspace
            .descendants_from(&[DIR.into(), PATH.into()])
            .into_iter()
            .map(move |location| {
                (
                    Self::deserialize(
                        workspace.read(loc!([DIR, PATH, location.clone()] as W::Component)),
                    ),
                    location,
                )
            })
    }

    pub fn save<W: Workspace>(&self, workspace: &W, location: &[W::Component]) {
        let mut serializer = DeckSerializer::default();
        serializer.serialize_value(self).unwrap();

        let bytes = serializer.into_serializer().into_inner();
        workspace.write(loc!([DIR, PATH, location] as W::Component), &bytes[..]);
    }

    fn deserialize(bytes: Rc<[u8]>) -> Self {
        (bytes.cast() as Rc<ArchivedDeck>)
            .as_ref()
            .deserialize(&mut SharedDeserializeMap::new())
            .unwrap()
    }
}

impl From<Document> for Deck {
    fn from(value: Document) -> Self {
        let mut cards = Cards::new();
        for rem in value.rems().iter() {
            rem.insert_into(&mut cards, 0);
        }
        Self { cards }
    }
}

impl ArchivedDeck {
    pub fn get_card(&self, id: &str) -> Card {
        self.cards
            .get(dbg!(id))
            .unwrap()
            .deserialize(&mut SharedDeserializeMap::new())
            .unwrap()
    }

    pub fn load<W: Workspace>(workspace: &W, location: &[W::Component]) -> Rc<Self> {
        dbg!(workspace.read(loc!([DIR, PATH, location] as W::Component))).cast()
    }
}

type Rems = Vec<(usize, Vec<Content>)>;

#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive_attr(derive(Debug))]
pub struct Card {
    rems: Rems,
}

impl Card {
    pub fn groups(&self) -> HashSet<Group> {
        let mut set = HashSet::new();
        for (_, rem) in self.rems.iter() {
            for group in rem.iter().filter_map(Content::group) {
                set.insert(group);
            }
        }
        set
    }

    pub fn rems(&self) -> &Rems {
        &self.rems
    }
}

impl Extend<(usize, Vec<Content>)> for Card {
    fn extend<T: IntoIterator<Item = (usize, Vec<Content>)>>(&mut self, iter: T) {
        self.rems.extend(iter);
    }
}

impl<const N: usize> From<[(usize, Vec<Content>); N]> for Card {
    fn from(value: [(usize, Vec<Content>); N]) -> Self {
        Card {
            rems: Vec::from(value),
        }
    }
}

pub type CardId = (Rc<str>, Group);
