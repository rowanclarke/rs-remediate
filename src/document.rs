mod cards;
pub mod deserialize;
pub mod parser;
mod serialize;
pub use serialize::serialize;
use std::{
    collections::{BTreeMap, HashSet},
    fmt::{self, Display},
    rc::Rc,
};

use self::parser::{Content, Group};

const PATH: &str = "documents";

pub type Deck = BTreeMap<Rc<str>, Card>;
pub type Card = Vec<(usize, Vec<Content>)>;
pub type CardId = (Rc<str>, Group);

pub fn groups(card: &Card) -> HashSet<Group> {
    let mut set = HashSet::new();
    for (_, rem) in card {
        for group in rem.iter().filter_map(Content::group) {
            set.insert(group);
        }
    }
    set
}

pub enum DisplayCardStatus {
    Show,
    Hide,
}

pub struct DisplayCard {
    card: Card,
    group: Group,
    status: DisplayCardStatus,
}

impl DisplayCard {
    pub fn new(card: Card, group: Group) -> Self {
        Self {
            card,
            group,
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
        let mut cards = self.card.iter().peekable();
        while let Some((depth, rem)) = cards.next() {
            write!(f, "{}", "  ".repeat(*depth))?;
            for content in rem {
                match content {
                    Content::Closure(group, text) if group == &self.group => match &self.status {
                        DisplayCardStatus::Show => write!(f, "[{}]", text)?,
                        DisplayCardStatus::Hide => write!(f, "[...]")?,
                    },
                    Content::Text(text) | Content::Closure(_, text) => write!(f, "{}", text)?,
                }
            }
            if cards.peek().is_some() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
