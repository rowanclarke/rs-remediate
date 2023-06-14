mod document;
mod error;

use bincode::serialize_into;
use document::{parse, Closure, Content, Document, Group, Id, Rule, Text};
use error::SerializeError;
use std::{
    collections::BTreeMap,
    env::{self, join_paths},
    ffi::OsStr,
    fs::{read_to_string, File},
    path::Path,
};

type Deck<'a> = BTreeMap<(&'a str, &'a str), Vec<&'a str>>;

pub fn serialize(path: &Path) -> Result<(), SerializeError<Rule>> {
    let dir_out = &env::var("REMEDY_DIR").map_err(|_| SerializeError::EnvironmentError)?;
    let path_out = Path::new(dir_out).join(path.file_stem().unwrap_or(OsStr::new("")));
    let file_err = |e| SerializeError::FileError(e);
    let file_out = File::create(path_out).map_err(file_err)?;
    let str_in = read_to_string(path).map_err(file_err)?;
    let document = parse(str_in.as_str())?;
    serialize_into(file_out, &document.to_deck()).map_err(|_| SerializeError::SerializeError)?;
    Ok(())
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
