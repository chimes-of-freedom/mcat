//! Repository abstractions and concrete repository module declarations.

use crate::errors::McatResult;
use crate::models::TagAttributes;

pub mod toml_repo;

pub trait Repository {
    fn init_empty() -> Self
    where
        Self: Sized;

    fn insert_track(&mut self, file_hash: String, tag_attr: TagAttributes);

    fn persist(&self) -> McatResult<()>;
}
