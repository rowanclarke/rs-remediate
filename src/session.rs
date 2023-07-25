mod entry;

use crate::{
    archive::{impls::reverse::Reverse, with::AsBoxedSlice, Cast},
    deck::Deck,
    schedule::Review,
    workspace::{Component, Workspace},
    RemedyRoot,
};
pub use entry::Entry;
use rkyv::{
    de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer, ser::Serializer,
    Archive, Archived, Deserialize, Serialize,
};
use std::{collections::BinaryHeap, fmt::Debug};

#[derive(Debug, Archive, Deserialize, Serialize)]
#[archive(check_bytes)]
pub struct Session<C, D> {
    #[with(AsBoxedSlice<SessionQueueInner<C, D>>)]
    queue: SessionQueue<C, D>,
}

type SessionQueue<C, D> = BinaryHeap<SessionQueueInner<C, D>>;
type SessionQueueInner<C, D> = Reverse<Entry<C, D>>;
type SessionSerializer = AllocSerializer<1024>;

impl<
        C: Component + Archive + Serialize<SessionSerializer>,
        D: Review + Archive + Serialize<SessionSerializer>,
    > Session<C, D>
where
    Archived<C>: Deserialize<C, SharedDeserializeMap>,
    Archived<D>: Deserialize<D, SharedDeserializeMap>,
{
    pub fn new<W: Workspace<Component = C>>(workspace: &W) -> Self {
        let mut queue = BinaryHeap::<Reverse<Entry<C, D>>>::new();
        for (deck, path) in Deck::all(workspace) {
            for (id, card) in deck.cards() {
                for group in card.groups() {
                    queue.push(Reverse::new(Entry::<C, D>::new(
                        path.clone(),
                        (id.clone(), group),
                        D::default(),
                    )));
                }
            }
        }
        Self { queue }
    }

    pub fn save<W: Workspace<Component = C>>(&self, workspace: &W) {
        let mut serializer = SessionSerializer::default();
        serializer.serialize_value(self).unwrap();
        let bytes = serializer.into_serializer().into_inner();
        workspace.write::<RemedyRoot, _>("session", &bytes[..]);
    }

    pub fn load<W: Workspace<Component = C>>(workspace: &W) -> Self {
        (workspace.read::<RemedyRoot, _>("session").as_ref().cast() as &Archived<Self>)
            .deserialize(&mut SharedDeserializeMap::new())
            .unwrap()
    }

    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Entry<C, D>) -> bool,
    {
        while let Some(mut entry) = self.queue.pop().map(Reverse::get) {
            let stop = f(&mut entry);
            self.queue.push(Reverse::new(entry));
            if stop {
                break;
            }
        }
    }
}
