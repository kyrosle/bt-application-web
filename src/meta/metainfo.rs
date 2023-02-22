use std::path::PathBuf;
use url::Url;

pub type Sha1Hash = [u8; 20];

pub struct FileInfo {
  pub path: PathBuf,
  pub len: u64,
  pub torrent_offset: u64,
}

pub struct Metainfo {
  pub name: String,
  pub info_hash: Sha1Hash,
  pub pieces: Vec<u8>,
  pub piece_len: u32,
  pub files: Vec<FileInfo>,
  pub trackers: Vec<Url>,
}
