//! Functions associated with table "images".

use anyhow::Result;
use rusqlite::Transaction;

use crate::{
    common,
    models::{Image, NewImage},
    repos::TrackRepo,
};

pub struct TableImages;

impl TableImages {
    pub(super) fn insert_one(tx: &Transaction, image: NewImage) -> Result<Image> {
        let image_hash = common::compute_data_hash(&image.data);
        let image_id = TrackRepo::insert_or_get_id(
            tx,
            "INSERT OR IGNORE INTO images (hash, data) VALUES (?1, ?2)",
            (&image_hash, &image.data),
            "SELECT id FROM images WHERE hash = ?1",
            (&image_hash,),
        )?;

        Ok(Image::from_new(image, image_id, image_hash))
    }
}
