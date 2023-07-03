mod entry;

use crate::{
    document::{
        deserialize::{all, get},
        CardId, DisplayCard,
    },
    file::{create, open, read},
    schedule::{Query, Review},
    with::AsBoxedSlice,
};
use entry::Entry;
use rkyv::{
    archived_root, de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer,
    ser::Serializer, Archive, Archived, Deserialize, Serialize,
};
use std::{collections::BinaryHeap, fmt::Debug, io::Write};

const PATH: &str = "session";

#[derive(Debug, Archive, Deserialize, Serialize)]
#[archive(check_bytes)]
pub struct Session<Data> {
    #[with(AsBoxedSlice<Entry<Data>>)]
    queue: SessionQueue<Data>,
}

type SessionQueue<D> = BinaryHeap<Entry<D>>;
type SessionSerializer = AllocSerializer<1024>;

impl<D: Review + Archive + Serialize<SessionSerializer>> Session<D>
where
    Archived<D>: Deserialize<D, SharedDeserializeMap>,
{
    pub fn new() -> Self {
        let mut queue = BinaryHeap::<Entry<D>>::new();
        for (path, deck) in all() {
            for (id, _) in deck {
                queue.push(Entry::<D>::new(path.clone(), id, D::default()))
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
            if let Some((path, id, mut data)) = self.queue.pop().map(Entry::into_components) {
                let mut card = DisplayCard::new(get(path.clone(), unsafe {
                    archived_root::<CardId>(id.archived().as_slice())
                }));
                println!("{}", card);
                card.show();
                println!("{}", card);
                data.review(<D as Review>::Score::query());
                self.queue.push(Entry::<D>::new(path, id, data));
            }
        }
    }
}
