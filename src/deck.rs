mod cards;
pub mod document;
use crate::{
    archive::Cast,
    loc, loc_root, root,
    workspace::{AsComponents, Component, IntoComponents, Root, Workspace, WorkspaceRoot},
    RemedyRoot,
};
use rkyv::{ser::serializers::AllocSerializer, ser::Serializer, Archive, Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    rc::Rc,
    str::from_utf8,
};

pub use document::{Content, Group};
use rkyv::de::deserializers::SharedDeserializeMap;

use self::document::Document;

root!(pub type DeckRoot: RemedyRoot = ["decks"]);

type Cards = BTreeMap<Rc<str>, Card>;

type DeckSerializer = AllocSerializer<1024>;

#[derive(Archive, Serialize, Deserialize, Debug)]
#[archive_attr(derive(Debug))]
pub struct Deck {
    cards: Cards,
}

impl Deck {
    pub fn cards(&self) -> &Cards {
        &self.cards
    }

    pub fn all<W: Workspace>(
        workspace: &W,
    ) -> impl Iterator<Item = (Self, Rc<[W::Component]>)> + '_ {
        workspace
            .descendants_from::<DeckRoot, _>(())
            .into_iter()
            .map(move |location| {
                (
                    Self::deserialize(workspace.read::<DeckRoot, _>(location.clone())),
                    location,
                )
            })
    }

    pub fn save<W: Workspace>(&self, workspace: &W, location: &[W::Component]) {
        let mut serializer = DeckSerializer::default();
        serializer.serialize_value(self).unwrap();

        let bytes = serializer.into_serializer().into_inner();
        workspace.write::<DeckRoot, _>(location, &bytes[..]);
    }

    pub fn parse<W: Workspace>(workspace: &W, location: &[W::Component]) -> Self {
        let input = workspace.read::<WorkspaceRoot, _>(location);
        let document = Document::parse(from_utf8(input.as_ref()).unwrap()).unwrap();
        document.into()
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
            .get(id)
            .unwrap()
            .deserialize(&mut SharedDeserializeMap::new())
            .unwrap()
    }

    pub fn load<W: Workspace>(workspace: &W, location: &[W::Component]) -> Rc<Self> {
        workspace.read::<DeckRoot, _>(location).cast()
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
