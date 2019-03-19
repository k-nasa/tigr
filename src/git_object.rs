use crate::repository::Repository;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::prelude::*;

pub trait GitObject {
    fn serialize(&self) -> &str;
    fn deserialize(&mut self, data: &str) -> ();
}

pub struct GitBlob {
    repo: Repository,
    blobdate: String,
}
impl GitBlob {
    pub fn new(repo: Repository, blobdate: &str) -> Self {
        GitBlob {
            repo,
            blobdate: blobdate.into(),
        }
    }
}

impl GitObject for GitBlob {
    fn serialize(&self) -> &str {
        &self.blobdate
    }

    fn deserialize(&mut self, data: &str) {
        self.blobdate = data.to_string();
    }
}
