use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compression {
    Unknown = 0x00,
    None = 0x01,
    Gzip = 0x02,
    Brotli = 0x03,
    Zstd = 0x04,
}

impl TryFrom<u8> for Compression {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Compression::Unknown),
            0x01 => Ok(Compression::None),
            0x02 => Ok(Compression::Gzip),
            0x03 => Ok(Compression::Brotli),
            0x04 => Ok(Compression::Zstd),
            _ => Err("Invalid compression value"),
        }
    }
}

impl fmt::Display for Compression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Unknown = 0x00,
    MVT = 0x01,
    PNG = 0x02,
    JPEG = 0x03,
    WebP = 0x04,
    AVIF = 0x05,
}

impl TryFrom<u8> for TileType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TileType::Unknown),
            0x01 => Ok(TileType::MVT),
            0x02 => Ok(TileType::PNG),
            0x03 => Ok(TileType::JPEG),
            0x04 => Ok(TileType::WebP),
            0x05 => Ok(TileType::AVIF),
            _ => Err("Invalid tile type value"),
        }
    }
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}