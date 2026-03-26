use lofty::prelude::*;
use lofty::tag::Tag;

pub fn display_tag(primary_tag: &Tag) {
    println!("==== Tag Info ====");
    println!(
        "Title: {}",
        primary_tag.title().as_deref().unwrap_or("None")
    );
    println!(
        "Artist: {}",
        primary_tag.artist().as_deref().unwrap_or("None")
    );
    println!(
        "Album: {}",
        primary_tag.album().as_deref().unwrap_or("None")
    );
    println!(
        "Album Artist: {}",
        primary_tag
            .get_string(ItemKey::AlbumArtist)
            .unwrap_or("None"),
    );
    println!(
        "Genre: {}",
        primary_tag.genre().as_deref().unwrap_or("None")
    );
    println!(
        "Album Artist: {}",
        primary_tag
            .get_string(ItemKey::AlbumArtist)
            .unwrap_or("None"),
    );
}
