pub mod utils;

#[derive(Debug)]
pub enum McatError {
    FileNotFound,
    OpenFailed,
    ReadFailed,
    TagNotFound,
    AttrNotFound,
}