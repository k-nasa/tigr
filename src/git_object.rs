use crate::repository::Repository;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

pub trait GitObject {
    fn serialize(&self) -> &str;
    fn deserialize(&mut self, data: &str) -> ();
    fn repo(&self) -> PathBuf {
        self.repo()
    }
    fn fmt(&self) -> String {
        self.fmt()
    }
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

    pub fn fmt() -> String {
        String::from("blob")
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

pub fn read_object(repo: Repository, sha: &str) -> Result<Box<dyn GitObject>, failure::Error> {
    let path = repo
        .gitdir()
        .join("objects")
        .join(&sha[0..2])
        .join(&sha[2..]);

    let zlib_string = fs::read_to_string(path)?;
    let mut z = ZlibDecoder::new(zlib_string.as_bytes());
    let mut raw = String::new();
    z.read_to_string(&mut raw)?;

    // Read object type
    let x = raw.find(' ').unwrap();
    let fmt = &raw[0..x];

    // Read and validate object size
    let y = raw.find('\x00').unwrap();
    let size: usize = raw[x..y].parse()?;

    if size != raw.len() - y - 1 {
        failure::bail!("Malformed object {}: bad length", sha)
    }

    match fmt {
        "commit" => unimplemented!(),
        "tree" => unimplemented!(),
        "tag" => unimplemented!(),
        "blob" => Ok(Box::new(GitBlob::new(repo, &raw[y + 1..]))),
        _ => failure::bail!("Unknown type {} for object {}", fmt, sha),
    }
}

pub fn write_object(
    object: impl GitObject,
    actually_write: bool,
) -> Result<String, failure::Error> {
    let data = object.serialize();
    let result = format!("{}{}{}{}{}", object.fmt(), b' ', data.len(), b'\x00', data);

    let sha = s_to_sha(&result);

    if actually_write {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(sha.as_bytes())?;
        let compressed_bytes = e.finish()?;

        fs::write(
            object
                .repo()
                .join("objects")
                .join(&sha[0..2])
                .join(&sha[2..]),
            compressed_bytes,
        )?;
    }

    Ok(sha)
}

fn s_to_sha(s: &str) -> String {
    let mut m = sha1::Sha1::new();
    m.update(s.as_bytes());
    m.digest().to_string()
}
