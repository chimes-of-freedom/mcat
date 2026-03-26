use crate::{McatError, TagAttributes};

use std::path::Path;

use lofty::prelude::*;
use lofty::{config::WriteOptions, tag::Tag};

pub fn edit_tag<P: AsRef<Path>>(
    output_path: P,
    tag: &mut Tag,
    tag_attr: TagAttributes,
) -> Result<(), McatError> {
    if tag_attr.is_empty() {
        return Err(McatError::AttrEmpty);
    }

    if let Some(title) = tag_attr.title {
        tag.set_title(title);
    }
    if let Some(artist) = tag_attr.artist {
        tag.set_artist(artist);
    }
    if let Some(album) = tag_attr.album {
        tag.set_album(album);
    }
    if let Some(genre) = tag_attr.genre {
        tag.set_genre(genre);
    }

    tag.save_to_path(output_path.as_ref(), WriteOptions::default())?;

    Ok(())
}
