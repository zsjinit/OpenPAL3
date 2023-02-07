use crate::fs::plain_fs::PlainFs;
use memmap::{Mmap, MmapOptions};
use std::{fs::File, io::Cursor, path::Path};

use super::sfb_archive::SfbArchive;

pub type SfbFs = PlainFs<SfbArchive<Mmap>>;

impl SfbFs {
    pub fn create<P: AsRef<Path>>(sfb_path: P) -> anyhow::Result<SfbFs> {
        let file = File::open(sfb_path.as_ref())?;
        let mem = unsafe { MmapOptions::new().map(&file)? };
        let cursor = Cursor::new(mem);
        let sfb_archive = SfbArchive::load(cursor)?;
        Ok(Self::new(sfb_archive))
    }
}
