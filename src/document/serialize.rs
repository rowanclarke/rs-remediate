mod error;

use crate::REMEDY_DIR;

use super::{
    parser::{parse, Document, Rule},
    PATH,
};
pub use error::SerializeError;
use rkyv::{
    ser::{
        serializers::{AlignedSerializer, AllocScratch, CompositeSerializer, SharedSerializeMap},
        Serializer,
    },
    util::AlignedVec,
};
use std::{
    env,
    ffi::OsStr,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

type DocumentSerializer =
    CompositeSerializer<AlignedSerializer<AlignedVec>, AllocScratch, SharedSerializeMap>;

pub fn serialize(path: &Path) -> Result<(), SerializeError<Rule>> {
    let document = document_in(path)?;
    let mut serializer = DocumentSerializer::default();
    serializer.serialize_value(&document.deck()).unwrap();

    let bytes = serializer.into_serializer().into_inner();
    file_out(path)?
        .write_all(&bytes[..])
        .map_err(|_| SerializeError::SerializeError)
}

fn document_in(path: &Path) -> Result<Document, SerializeError<Rule>> {
    let str_in = read_to_string(path).map_err(|e| SerializeError::FileError(e))?;
    parse(str_in.as_str())
}

fn file_out(path: &Path) -> Result<File, SerializeError<Rule>> {
    let rem_dir = &env::var(REMEDY_DIR).map_err(|_| SerializeError::EnvironmentError)?;
    let path_out = Path::new(&rem_dir)
        .join(PATH)
        .join(path.file_stem().unwrap_or(OsStr::new("")));
    File::create(path_out).map_err(|e| SerializeError::FileError(e))
}
