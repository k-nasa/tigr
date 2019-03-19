use crate::repository::Repository;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::prelude::*;

pub trait GitObject {
    fn serialize(&self) -> &str;
    fn deserialize(&mut self, data: &str) -> ();
}
