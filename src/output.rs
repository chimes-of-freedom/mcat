//! Output formatting and presentation helpers for terminal display.

use tabled::{Table, settings::Style};

use crate::models::TagAttributes;

pub fn display_tag_attrs(tag_attrs: &[&TagAttributes]) {
    let mut table = Table::new(tag_attrs);

    table.with(Style::modern());

    println!("{table}");
}
