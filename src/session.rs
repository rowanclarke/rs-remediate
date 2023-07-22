mod entry;

use crate::{
    archive::{impls::reverse::Reverse, with::AsBoxedSlice},
    document::{
        deserialize::{card, decks},
        groups,
        parser::Group,
        Card, DisplayCard,
    },
    file::{create, open, read},
    schedule::{Query, Review},
};
use entry::Entry;
use rkyv::{
    archived_root, de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer,
    ser::Serializer, with::With, Archive, Archived, Deserialize, Serialize,
};
use std::{collections::BinaryHeap, fmt::Debug, io::Write};

const PATH: &str = "session";

#[derive(Debug, Archive, Deserialize, Serialize)]
#[archive(check_bytes)]
pub struct Session<Data> {
    #[with(AsBoxedSlice<SessionQueueInner<Data>>)]
    queue: SessionQueue<Data>,
}

type SessionQueue<D> = BinaryHeap<SessionQueueInner<D>>;
type SessionQueueInner<D> = Reverse<Entry<D>>;
type SessionSerializer = AllocSerializer<1024>;

impl<D: Review + Archive + Serialize<SessionSerializer>> Session<D>
where
    Archived<D>: Deserialize<D, SharedDeserializeMap>,
{
    pub fn new() -> Self {
        let mut queue = BinaryHeap::<Reverse<Entry<D>>>::new();
        for (path, deck) in decks() {
            for (id, card) in deck {
                for group in groups(&card) {
                    queue.push(Reverse::new(Entry::<D>::new(
                        path.clone(),
                        (id.clone(), group),
                        D::default(),
                    )));
                }
            }
        }
        Self { queue }
    }

    pub fn save(&self) {
        let mut serializer = SessionSerializer::default();
        serializer.serialize_value(self).unwrap();
        let bytes = serializer.into_serializer().into_inner();
        create(&[PATH]).write_all(&bytes[..]).unwrap();
    }

    pub fn load() -> Self {
        unsafe { archived_root::<Self>(read(open(&[PATH])).as_slice()) }
            .deserialize(&mut SharedDeserializeMap::new())
            .unwrap()
    }

    pub fn learn(&mut self) {
        loop {
            if let Some((path, (id, group), mut data)) = self
                .queue
                .pop()
                .map(Reverse::to_owned)
                .map(Entry::into_components)
            {
                let mut card = DisplayCard::new(card(path.clone(), id.clone()), group.clone());
                println!("{}", card);
                card.show();
                println!("{}", card);
                data.review(<D as Review>::Score::query());
                self.queue
                    .push(Reverse::new(Entry::<D>::new(path, (id, group), data)));
            }
        }
    }
}