pub mod metainfo;
pub mod trackerinfo;

pub trait TestMeta {
  fn create_for_test() -> Self;
}