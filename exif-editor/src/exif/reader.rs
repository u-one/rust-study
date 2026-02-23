use std::path::Path;

use chrono::{DateTime, Local, NaiveDateTime};
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;

use super::types::{ExifData, GpsCoordinate};

pub struct FileTimestamps {
    pub created: Option<DateTime<Local>>,
    pub modified: Option<DateTime<Local>>,
}

pub fn read_file_timestamps(path: &Path) -> FileTimestamps {
    let mut timestamps = FileTimestamps {
        created: None,
        modified: None,
    };

    if let Ok(metadata) = std::fs::metadata(path) {
        if let Ok(created) = metadata.created() {
            timestamps.created = Some(DateTime::from(created));
        }
        if let Ok(modified) = metadata.modified() {
            timestamps.modified = Some(DateTime::from(modified));
        }
    }

    timestamps
}

pub fn read_exif(path: &Path) -> ExifData {
    let mut exif_data = ExifData::default();

    let Ok(metadata) = Metadata::new_from_path(path) else {
        return exif_data;
    };

    // 撮影日時を読み取り
    for tag in metadata.get_tag(&ExifTag::DateTimeOriginal(String::new())) {
        if let ExifTag::DateTimeOriginal(datetime_str) = tag {
            if let Some(dt) = parse_exif_datetime(datetime_str) {
                exif_data.date = Some(dt.date());
                exif_data.time = Some(dt.time());
                break;
            }
        }
    }

    // GPS座標を読み取り
    let lat = get_gps_coordinate(&metadata, true);
    let lat_ref = get_gps_ref(&metadata, true);

    if let (Some(lat_val), Some(lat_r)) = (lat, lat_ref) {
        exif_data.latitude = Some(GpsCoordinate::new(lat_val, &lat_r == "N"));
    }

    let lon = get_gps_coordinate(&metadata, false);
    let lon_ref = get_gps_ref(&metadata, false);

    if let (Some(lon_val), Some(lon_r)) = (lon, lon_ref) {
        exif_data.longitude = Some(GpsCoordinate::new(lon_val, &lon_r == "E"));
    }

    exif_data
}

fn get_gps_ref(metadata: &Metadata, is_latitude: bool) -> Option<String> {
    if is_latitude {
        for tag in metadata.get_tag(&ExifTag::GPSLatitudeRef(String::new())) {
            if let ExifTag::GPSLatitudeRef(s) = tag {
                return Some(s.clone());
            }
        }
    } else {
        for tag in metadata.get_tag(&ExifTag::GPSLongitudeRef(String::new())) {
            if let ExifTag::GPSLongitudeRef(s) = tag {
                return Some(s.clone());
            }
        }
    }
    None
}

fn get_gps_coordinate(metadata: &Metadata, is_latitude: bool) -> Option<f64> {
    if is_latitude {
        for tag in metadata.get_tag(&ExifTag::GPSLatitude(vec![])) {
            if let ExifTag::GPSLatitude(values) = tag {
                if values.len() >= 3 {
                    let degrees = rational_to_f64(&values[0]);
                    let minutes = rational_to_f64(&values[1]);
                    let seconds = rational_to_f64(&values[2]);
                    return Some(degrees + minutes / 60.0 + seconds / 3600.0);
                }
            }
        }
    } else {
        for tag in metadata.get_tag(&ExifTag::GPSLongitude(vec![])) {
            if let ExifTag::GPSLongitude(values) = tag {
                if values.len() >= 3 {
                    let degrees = rational_to_f64(&values[0]);
                    let minutes = rational_to_f64(&values[1]);
                    let seconds = rational_to_f64(&values[2]);
                    return Some(degrees + minutes / 60.0 + seconds / 3600.0);
                }
            }
        }
    }
    None
}

fn rational_to_f64(r: &little_exif::rational::uR64) -> f64 {
    r.nominator as f64 / r.denominator as f64
}

fn parse_exif_datetime(s: &str) -> Option<NaiveDateTime> {
    // EXIF形式: "YYYY:MM:DD HH:MM:SS"
    NaiveDateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S").ok()
}

pub fn scan_jpg_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if ext_lower == "jpg" || ext_lower == "jpeg" {
                        files.push(path);
                    }
                }
            }
        }
    }

    files.sort();
    files
}
