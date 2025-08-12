use super::super::vector_core::store::VectorStore;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{rename, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub store: VectorStore,
}

impl Snapshot {
    pub fn save_to_disk<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let tmp_path = path.as_ref().with_extension("tmp");
        let tmp_file = File::create(&tmp_path)?;
        let mut writer = BufWriter::new(tmp_file);

        serde_json::to_writer(&mut writer, &self.store)?;

        writer.flush()?;
        writer.get_ref().sync_all()?;

        rename(&tmp_path, &path)?;

        if let Some(parent) = path.as_ref().parent() {
            let dir_file = File::open(parent)?;
            dir_file.sync_all()?;
        }

        Ok(())
    }

    pub fn load_from_disk<P: AsRef<Path>>(path: P) -> Result<VectorStore, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let store = serde_json::from_reader(reader)?;

        Ok(store)
    }
}
