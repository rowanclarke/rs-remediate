mod entry;

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fmt::Debug,
};

use crate::{
    document::{self, all, CardId, Segment},
    schedule::{self, Query, Review},
};
use entry::Entry;

const PATH: &str = "session";

#[derive(Debug)]
pub struct Session<D>(SessionHeap<D>);
type SessionHeap<D> = BinaryHeap<Reverse<Entry<D>>>;

impl<D: Review + Debug> Session<D> {
    pub fn new() -> Self {
        let mut session = SessionHeap::<D>::new();
        for document in all() {
            for id in document.into_keys() {
                session.push(Reverse(Entry::new(id, D::default())));
            }
        }
        Session(session)
    }

    pub fn learn(&mut self) {
        let Session(session) = self;
        if let Some(card) = session.pop() {
            let (id, mut data) = card.0.unwrap();
            Self::with_value(&id, |vec| {
                Self::print_hide(vec);
                Self::print_show(vec);
                data.review(<D as Review>::Score::query());
            });
            session.push(Reverse(Entry::new(id, data)));
        }
    }

    fn print_hide(vec: &Vec<Segment>) {
        for segment in vec {
            match &segment {
                Segment::Visible(text) => print!("{}", text),
                Segment::Hidden(_) => print!("[...]"),
            }
        }
        println!();
    }

    fn print_show(vec: &Vec<Segment>) {
        for segment in vec {
            match &segment {
                Segment::Visible(text) => print!("{}", text),
                Segment::Hidden(text) => print!("[{}]", text),
            }
        }
        println!();
    }

    fn with_value(id: &CardId, mut f: impl FnMut(&Vec<Segment>)) {
        for document in all() {
            if let Some(value) = document.get(id) {
                f(value)
            }
        }
    }
}
