use std::io::{self, Read};
use flate2::read::GzDecoder;
use std::fmt;

use crate::protobufs::decode_varint;
use crate::tileid::TileId;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    delta_encoded_tileid: u64,
    pub tileid: TileId,
    pub run_length: usize,
    pub length: usize,
    pub offset: usize,
}

#[allow(unused)]
impl DirectoryEntry {
    pub fn print_info(&self) {
        println!("{:?}", self);
    }
}
impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (z, x, y) = self.tileid.decode();

        write!(f, "DirectoryEntry delta-encoded TileID:{} TileID:{} z,x,y:{},{},{} RunLength:{} Length:{} Offset:{}", 
            self.delta_encoded_tileid,
                self.tileid.value(),
                z, x, y,
                self.run_length,
                self.length,
                self.offset,
        )
    }
}

#[derive(Debug)]
pub struct Directory {
    pub entries: Vec<DirectoryEntry>,
}

impl Directory {
    pub fn parse_compressed(data: &[u8]) -> io::Result<Self> {
        let mut decoder = GzDecoder::new(data);
        let mut data_uncompressed = Vec::new();
        decoder.read_to_end(&mut data_uncompressed)?;

        Self::parse(&data_uncompressed)
    }

    pub fn parse(data: &[u8]) -> io::Result<Self> {
        let (value, mut offset) = decode_varint(&data)
            .expect("Number of entries is encoded as a little-endian varible-width integer");
        let num_of_entries = value;

        println!("Number of Entries: {}", num_of_entries);

        let (delta_encoded_tileids, size) = read_varints(&data[offset..], num_of_entries)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        offset += size;

        let (run_lengths, size) = read_varints(&data[offset..], num_of_entries)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        offset += size;

        let (lengths, size) = read_varints(&data[offset..], num_of_entries)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        offset += size;

        let (offsets, _) = read_varints(&data[offset..], num_of_entries)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut entries = Vec::with_capacity(num_of_entries as usize);
        let mut last_tile_id: u64 = 0;

        let mut last_offset: u64 = 0;
        let mut last_length: u64 = 0;

        for i in 0..num_of_entries as usize {
            let tile_id_value = last_tile_id + delta_encoded_tileids[i];

            let current_raw_offset = offsets[i];
            let actual_offset = if current_raw_offset == 0 && i > 0 {
                last_offset + last_length
            } else {
                current_raw_offset.saturating_sub(1)
            };

            entries.push(DirectoryEntry {
                delta_encoded_tileid: delta_encoded_tileids[i],
                tileid: TileId::new(tile_id_value),
                run_length: run_lengths[i] as usize,
                length: lengths[i] as usize,
                offset: actual_offset as usize,
            });
            last_tile_id = tile_id_value;
            last_offset = actual_offset;
            last_length = lengths[i];
        }

        Ok(Directory {entries})

    }
}

fn read_varints(buffer: &[u8], count: u64) -> Result<(Vec<u64>, usize), &'static str> {
    let mut values: Vec<u64> = Vec::new();
    let mut size_read: usize = 0;
    for _ in 0..count {
        let (value, size) = decode_varint(&buffer[(size_read)..])?;
        //println!("Value: {}, Bytes read: {}", value, size);
        values.push(value);
        size_read += size;
    }
    Ok((values, size_read))

}

#[cfg(test)]
mod tests {
    use super::*;

    const DIR_DATA: [u8; 17] = [
        0x04, // Number of entries: 3
        0x01, 0x02, 0x03, 0x04, // Delta-encoded Tile IDs
        0x00, 0x01, 0x00, 0x00, // Run Lengths
        10, 20, 30, 40, // Lengths
        11, 0, 51, 1 // Offsets
    ]; 

    #[test]
    fn test_parse_directory() {
        let directory = Directory::parse(&DIR_DATA).expect("should parse directory data");
        assert_eq!(directory.entries.len(), 4);

        assert_eq!(directory.entries[0], DirectoryEntry {
            delta_encoded_tileid: 1,
            tileid: TileId::new(1),
            run_length: 0,
            length: 10,
            offset: 10,
        });

        assert_eq!(directory.entries[1], DirectoryEntry {
            delta_encoded_tileid: 2,
            tileid: TileId::new(3),
            run_length: 1,
            length: 20,
            offset: 20,
        });

        assert_eq!(directory.entries[2], DirectoryEntry {
            delta_encoded_tileid: 3,
            tileid: TileId::new(6),
            run_length: 0,
            length: 30,
            offset: 50,
        });

        assert_eq!(directory.entries[3], DirectoryEntry {
            delta_encoded_tileid: 4,
            tileid: TileId::new(10),
            run_length: 0,
            length: 40,
            offset: 0,
        });


    }
}