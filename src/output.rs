//! Output formatting and presentation helpers for terminal display.

use tabled::{Table, Tabled, settings::Style};

/// Displays a formatted table of an iterator whose `Item` implements
/// [`Tabled`] trait.
pub fn display_as_table<I, T>(tag_attrs: I)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    let mut table = Table::new(tag_attrs);

    table.with(Style::modern());

    println!("{table}");
}
