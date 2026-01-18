use std::io;

use super::types::{Compression, TileType};

const MAGIC_NUMBER: &[u8] = b"PMTiles";
const HEADER_SIZE: usize = 127;

fn to_u64_le(bytes: &[u8]) -> u64 {
    // sliceの場合は一度try_intoで配列に変換する必要がある
    u64::from_le_bytes(bytes.try_into().expect("fail to convert to u64"))
}

fn to_lat_lon(bytes: &[u8; 8]) -> (f64, f64) {
    let lon  = i32::from_le_bytes(bytes[0..4].try_into().expect("fail to convert to u64"));
    let lat  = i32::from_le_bytes(bytes[4..8].try_into().expect("fail to convert to u64"));
    let lon = lon as f64 / 10_000_000.0;
    let lat = lat as f64 / 10_000_000.0;
    (lon, lat)
}

#[derive(Debug, Clone)]
pub struct Header {
    pub version: u8,
    pub root_dir_offset: usize,
    pub root_dir_length: usize,
    pub metadata_offset: usize,
    pub metadata_length: usize,
    pub leaf_dirs_offset: usize,
    pub leaf_dirs_length: usize,
    pub tile_data_offset: usize,
    pub tile_data_length: usize,
    pub num_addressed_tiles: u64,
    pub num_tile_entries: u64,
    pub num_tile_contents: u64,
    pub clustered: u8,
    pub internal_compression: Compression,
    pub tile_compression: Compression,
    pub tile_type: TileType,
    pub min_zoom: u8,
    pub max_zoom: u8,
    pub min_position: (f64, f64),
    pub max_position: (f64, f64),
    pub center_zoom: u8,
    pub center_position: (f64, f64),
}

impl Header {
    pub fn parse(data: &[u8]) -> io::Result<Self> {
        if data.len() < HEADER_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Header too short: {} bytes", data.len())
            ));
        }

        let magic_number = &data[0x00..0x07];
        match str::from_utf8(magic_number) {
            Ok(s) => println!("Magic Number:{}",s),
            Err(e) => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid magic number: {}", e)
            )),
        }
        if magic_number != MAGIC_NUMBER {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid magic number: expected {:?}, found {:?}", MAGIC_NUMBER, magic_number)
            ));
        }

        let version = data[0x07];
        let root_dir_offset  = to_u64_le(&data[0x08..0x10]) as usize;
        let root_dir_length = to_u64_le(&data[0x10..0x18]) as usize;
        let metadata_offset = to_u64_le(&data[0x18..0x20]) as usize;
        let metadata_length = to_u64_le(&data[0x20..0x28]) as usize;
        let leaf_dirs_offset = to_u64_le(&data[0x28..0x30]) as usize;
        let leaf_dirs_length = to_u64_le(&data[0x30..0x38]) as usize;
        let tile_data_offset = to_u64_le(&data[0x38..0x40]) as usize;
        let tile_data_length = to_u64_le(&data[0x40..0x48]) as usize;
        let num_addressed_tiles: u64 = to_u64_le(&data[0x48..0x50]);
        let num_tile_entries: u64 = to_u64_le(&data[0x50..0x58]);
        let num_tile_contents: u64 = to_u64_le(&data[0x58..0x60]);
        let clustered : u8 = data[0x60];
        let internal_compression : Compression = data[0x61].try_into().expect("invalid compression value");
        let tile_compression : Compression = data[0x62].try_into().expect("invalid compression value");
        let tile_type : TileType = data[0x63].try_into().expect("invalid tile type value");
        let min_zoom : u8 = data[0x64];
        let max_zoom : u8 = data[0x65];
        let min_position  = to_lat_lon(&data[0x66..0x6E].try_into().expect("slice with incorrect length"));
        let max_position  = to_lat_lon(&data[0x6E..0x76].try_into().expect("slice with incorrect length"));
        let center_zoom : u8 = data[0x76];
        let center_position  = to_lat_lon(&data[0x77..0x7F].try_into().expect("slice with incorrect length"));

        Ok(Header {
            version: version,
            root_dir_offset: root_dir_offset,
            root_dir_length: root_dir_length,
            metadata_offset: metadata_offset,
            metadata_length: metadata_length,
            leaf_dirs_offset: leaf_dirs_offset,
            leaf_dirs_length: leaf_dirs_length,
            tile_data_offset: tile_data_offset,
            tile_data_length: tile_data_length,
            num_addressed_tiles: num_addressed_tiles,
            num_tile_entries: num_tile_entries,
            num_tile_contents: num_tile_contents,
            clustered: clustered,
            internal_compression: internal_compression,
            tile_compression: tile_compression,
            tile_type: tile_type,
            min_zoom: min_zoom,
            max_zoom: max_zoom,
            min_position: min_position,
            max_position: max_position,
            center_zoom: center_zoom,
            center_position: center_position, 
        })
        
    }

    pub fn print_info(&self) {
        println!("PMTiles Header:");
        println!("  Version: {}", self.version);
        println!("  Root Directory: offset={}, length={}", self.root_dir_offset, self.root_dir_length);
        println!("  Metadata: offset={}, length={}", self.metadata_offset, self.metadata_length);
        println!("  Leaf Directories: offset={}, length={}", self.leaf_dirs_offset, self.leaf_dirs_length);
        println!("  Tile Data: offset={}, length={}", self.tile_data_offset, self.tile_data_length);
        println!("  Tiles: addressed={}, entries={}, contents={}", 
            self.num_addressed_tiles, self.num_tile_entries, self.num_tile_contents);
        println!("  Clustered: {}", self.clustered);
        println!("  Compression: internal={}, tile={}", self.internal_compression, self.tile_compression);
        println!("  Tile Type: {}", self.tile_type);
        println!("  Zoom: min={}, max={}", self.min_zoom, self.max_zoom);
        println!("  Bounds: ({:.6}, {:.6}) - ({:.6}, {:.6})", 
            self.min_position.0, self.min_position.1,
            self.max_position.0, self.max_position.1);
        println!("  Center: ({:.6}, {:.6}), zoom={}", self.center_position.0, self.center_position.1, self.center_zoom);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const HEADER_DATA: [u8; 127] = [
        0x50, 0x4d, 0x54, 0x69, 0x6c, 0x65, 0x73, 0x03, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x45, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x1a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x8c, 0xd8, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0xf2, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x95, 0x51, 0x3d, 0xef, 0x03, 0x00, 0x00, 0x00, 0x44, 0xfe, 0x2c, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x47, 0x27, 0x2c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x64, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x02, 0x02, 0x01, 0x04, 0x10, 0x00, 0xb9, 0xb7, 0x48, 0xe8, 0x54, 0x27, 0x0a, 0xee, 0x84,
        0x3f, 0x5c, 0x00, 0x0b, 0x6b, 0x1b, 0x10, 0xa2, 0x25, 0xd3, 0x50, 0xbd, 0x92, 0xc2, 0x14,
    ];

    #[test]
    fn parse_correct_header() {
        let header = Header::parse(&HEADER_DATA)
            .expect("parse header successful");
        assert_eq!(header.version, 3);
        assert_eq!(header.root_dir_offset, 127);
        assert_eq!(header.root_dir_length, 2373);
        assert_eq!(header.metadata_offset, 2500);
        assert_eq!(header.metadata_length, 4166);
        assert_eq!(header.leaf_dirs_offset, 6666);
        assert_eq!(header.leaf_dirs_length, 5757068);
        assert_eq!(header.tile_data_offset, 5763734);
        assert_eq!(header.tile_data_length, 16898675093);
        assert_eq!(header.num_addressed_tiles, 2948676);
        assert_eq!(header.num_tile_entries, 2893639);
        assert_eq!(header.num_tile_contents, 2581507);
        assert_eq!(header.clustered, 1);
        assert_eq!(header.internal_compression, Compression::Gzip);
        assert_eq!(header.tile_compression, Compression::Gzip);
        assert_eq!(header.tile_type, TileType::MVT);
        assert_eq!(header.min_zoom, 4);
        assert_eq!(header.max_zoom, 16);
        assert_eq!(header.min_position, (122.0, 17.03498));
        assert_eq!(header.max_position, (154.766667, 46.0));
        assert_eq!(header.center_zoom, 16);
        assert_eq!(header.center_position, (135.601501, 34.8295869));
    }
    
}