
pub fn decode_varint(data: &[u8]) -> Result<(u64, usize), &'static str> {
    let mut result = 0u64;
    let mut shift = 0;

    for (i, &byte) in data.iter().enumerate() {
        if shift >= 64 {
            return Err("Varint is too long");
        }
        //println!("byte: {:08b}", byte);

        let value = (byte & 0b0111_1111) as u64;
        result |= value << shift;

        if byte & 0b1000_0000 == 0 {
            return Ok((result, i + 1));
        }
        shift += 7;
    } 
    Err("Incomplete varint data")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_150() {
        let bytes  = [0b1001_0110_u8, 0b0000_0001_u8, 0b0_u8];
        let result = decode_varint(&bytes);
        assert_eq!(result, Ok((150, 2)));
    }
}