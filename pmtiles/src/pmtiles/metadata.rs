use std::io::{self, Read};

use flate2::read::GzDecoder;
use super::types::TileType;
use serde_json::Value;

#[derive(Debug, PartialEq, Eq)]
pub struct Metadata {
    json: String,
}

#[allow(unused)]
impl Metadata {
    pub fn parse_compressed(data: &[u8], tile_type: TileType) -> io::Result<Self> {
        let mut decoder = GzDecoder::new(data);
        let mut metadata_decoded = Vec::new();
        decoder.read_to_end(&mut metadata_decoded)?;

        Self::parse(metadata_decoded, tile_type)
    }

    pub fn parse(data: Vec<u8>, tile_type: TileType) -> io::Result<Self> {
        let metadata_str = String::from_utf8(data).expect("Metadata is valid UTF-8 string");

        if tile_type != TileType::MVT {
            return Ok(Metadata {json: metadata_str});
        }

        let v = serde_json::from_str::<Value>(&metadata_str)?;
        let name = &v["name"];
        let description = &v["description"];
        let attribution = &v["attribution"];
        let _type = &v["type"];
        let version = &v["version"];
        
        println!("name: {}", name);
        println!("description: {}", description);
        println!("attribution: {}", attribution);
        println!("type: {}", _type);
        println!("version: {}", version);

        //parse_optional(&v);
        
        Ok(Metadata {json: metadata_str})
    }

    pub fn print_info(&self) {
        println!("Metadata:\n{}", self.json);
    }
}

#[allow(unused)]
fn parse_optional(value: &Value) {
    let format = &value["format"];
    println!("format: {}", format);
    let generator = &value["generator"];
    println!("generator: {}", generator);
    let generator_options = &value["generator_options"];
    println!("generator_options: {}", generator_options);
    let maxzoom = &value["maxzoom"];
    println!("maxzoom: {}", maxzoom);
    let minzoom = &value["minzoom"];
    println!("minzoom: {}", minzoom);
    let tilestats = &value["tilestats"];
    let layer_count = &tilestats["layerCount"];
    let layers = &tilestats["layers"];
    println!("tilestats layerCount: {}", layer_count);
    match layers.as_array(){
        Some(array) => {
            array.iter().for_each(|layer| {
                print_key_values(layer);
            });
        },
        None => {
            println!("tilestats layers is not an array");
        }
    }

}

fn print_key_values(value: &Value) {
        match value.as_object() {
            Some(map) => {
                map.iter().for_each(|(key, value)| {
                    println!("Metadata Key: {}, Value: {}", key, value);
                })
            },
            None => {
                println!("Metadata is not a JSON object");
            }
        };
    }



#[cfg(test)]
mod tests {
    use super::*;

    const METADATA: &str = "{\"name\":\"optimal_bvmap-v1\",\"description\":\"最適化されたベクトルタイル地図\",\"version\":\"1.0.0\"}";

    #[test]
    fn test_parse() {
        let metadata = Metadata::parse(METADATA.as_bytes().to_vec(), TileType::MVT).unwrap();
        assert_eq!(metadata.json, METADATA);

    }
}