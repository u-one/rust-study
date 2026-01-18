use std::fs::File;
use std::io;
use memmap2::Mmap;

mod directory;
mod header;
mod metadata;
mod types;

use directory::Directory;
use metadata::Metadata;
use header::Header;
use crate::{binaries::print_binary, tileid::TileId};

#[allow(unused)]
#[derive(Debug)]
pub struct PMTiles {
    data: Mmap,
    pub header: Header,
    pub root_directory: Directory,
    pub metadata: Metadata,
}

#[allow(unused)]
impl PMTiles {
    pub fn open(file_path: &str) -> io::Result<Self> {
        let f = File::open(file_path)?;
        let mmap = unsafe { Mmap::map(&f)? };
        let pmtiles = PMTiles::parse(mmap)?;
        Ok(pmtiles)
    }

    fn parse(data: Mmap) -> io::Result<Self> {
        let header = &data[..127];
        print_binary(&header);
        //print_binary_as_rust_code(&header);

        let header = Header::parse(&header)?;

        let compressed_root_dir = &data[header.root_dir_offset .. (header.root_dir_offset+header.root_dir_length)];
        let root_dir = Directory::parse_compressed(&compressed_root_dir)?;

        let compressed_metadata = &data[header.metadata_offset .. (header.metadata_offset+header.metadata_length)];
        let metadata = Metadata::parse_compressed(&compressed_metadata, header.tile_type)?;

        Ok(PMTiles {data: data, header: header, root_directory: root_dir, metadata: metadata} )
    }

    pub fn print_info(&self) {
        self.header.print_info();
        for entry in &self.root_directory.entries {
            println!("{}", entry);
        }
        self.metadata.print_info();
    }

    pub fn get(&self, z: u8, x: u32, y: u32)  {
        let tile_id = TileId::encode(z, x, y);

        println!("Get Tile z:{}, x:{}, y:{}, tile_id:{}", z, x, y, tile_id.value());

        let mut smallest_tile_id = TileId::new(0);
        let mut smallest_entry: Option<&directory::DirectoryEntry> = None;
        self.root_directory.entries.iter().for_each(|entry| {
            if entry.tileid.value() == tile_id.value() {
                println!("Found Tile in Directory Entry: {}", entry);
                if entry.run_length == 0 {
                    println!("Note: This entry has a run length of {}, meaning it covers multiple tiles starting from TileID {}", entry.run_length, entry.tileid.value());
                } else {
                    let tile_data_offset = entry.offset;
                    let tile_data_length = entry.length;
                    let tile_data = &self.data[tile_data_offset .. (tile_data_offset + tile_data_length)];
                    print_binary(&tile_data);
                }
                return;
            } else if (tile_id.value() < entry.tileid.value()) {
            } else if (tile_id.value() > entry.tileid.value()) {
                smallest_tile_id = entry.tileid;
                smallest_entry = Some(entry);
                println!("smallest_tile_id: {}", smallest_tile_id.value());
            }
        });
        match smallest_entry {
            Some(entry) => {
                if (entry.run_length) >= 1 {
                    let tile_data_offset = entry.offset;
                    let tile_data_length = entry.length;
                    let tile_data = &self.data[tile_data_offset .. (tile_data_offset + tile_data_length)];
                    print_binary(&tile_data);
                } else {
                    println!("leaf dirs offset: {}, entry offset: {}, length: {}", self.header.leaf_dirs_offset, entry.offset, entry.length);
                    let offset = self.header.leaf_dirs_offset + entry.offset;
                    let leaf_data = &self.data[offset .. (offset + entry.length)];
                    let  a = Directory::parse_compressed(leaf_data).expect("leaf directory");
                    a.entries.iter().for_each(|leaf_entry| {
                        println!("Leaf Directory Entry: {}", leaf_entry);
                        if leaf_entry.tileid.value() == tile_id.value() {
                            println!("Found Tile in Leaf Directory Entry: {}", leaf_entry);
                            let tile_data_offset = leaf_entry.offset;
                            let tile_data_length = leaf_entry.length;
                            let tile_data = &self.data[tile_data_offset .. (tile_data_offset + tile_data_length)];
                            print_binary(&tile_data);
                        }
                    });

                }
            },
            None => {
                println!("Tile not found and no higher TileID exists in the directory.");
            }
        }

    }

}

