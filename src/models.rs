//! Data types for the business logic and SQLite. All types are re-exported.

mod filter;
mod track;

pub use filter::*;
pub use track::*;

pub enum Patch {
    Keep,
    Set(String),
    Clear,
}
