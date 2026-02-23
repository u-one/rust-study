use crate::exif::types::GpsCoordinate;

pub fn parse_coordinate(input: &str) -> Option<f64> {
    input.trim().parse::<f64>().ok()
}

pub fn format_coordinate(coord: &GpsCoordinate) -> String {
    format!("{:.6}", coord.degrees)
}

pub fn format_decimal(value: f64) -> String {
    format!("{:.6}", value)
}
