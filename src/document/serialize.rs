use super::{
    parser::{parse, Document, Rule},
    PATH,
};
use crate::file;
use pest::error::Error;
use rkyv::{
    ser::{
        serializers::{AlignedSerializer, AllocScratch, CompositeSerializer, SharedSerializeMap},
        Serializer,
    },
    util::AlignedVec,
};
use std::{
    fs::{read_to_string},
    io::Write,
    path::Path,
};

type DocumentSerializer =
    CompositeSerializer<AlignedSerializer<AlignedVec>, AllocScratch, SharedSerializeMap>;

pub fn serialize(path: &Path) -> Result<(), Error<Rule>> {
    let document = document_in(path)?;
    let mut serializer = DocumentSerializer::default();
    serializer.serialize_value(&document.deck()).unwrap();

    let bytes = serializer.into_serializer().into_inner();
    file::create(&[PATH, file::stem(path)])
        .write_all(&bytes[..])
        .unwrap();
    Ok(())
}

fn document_in(path: &Path) -> Result<Document, Error<Rule>> {
    let str_in = read_to_string(path).unwrap();
    parse(str_in.as_str())
}
