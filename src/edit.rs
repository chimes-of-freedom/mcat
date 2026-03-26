use crate::{McatError, TagAttributes};

use std::path::Path;

use lofty::prelude::*;
use lofty::{config::WriteOptions, tag::Tag};

pub fn edit_tag<P: AsRef<Path>>(
    output_path: P,
    tag: &mut Tag,
    tag_attrs: TagAttributes,
) -> Result<(), McatError> {
    if tag_attrs.is_empty() {
        return Err(McatError::AttrEmpty);
    }

    if let Some(title) = tag_attrs.title {
        tag.set_title(title);
    }
    if let Some(artist) = tag_attrs.artist {
        tag.set_artist(artist);
    }
    if let Some(album) = tag_attrs.album {
        tag.set_album(album);
    }
    if let Some(genre) = tag_attrs.genre {
        tag.set_genre(genre);
    }

    tag.save_to_path(output_path.as_ref(), WriteOptions::default())
        .map_err(|_| McatError::WriteFailed)?;

    Ok(())
}
