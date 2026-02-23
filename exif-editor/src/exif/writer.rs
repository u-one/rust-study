use std::path::Path;

use chrono::{Datelike, Timelike};
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use little_exif::rational::uR64;

use super::types::ExifData;

pub fn write_exif(path: &Path, exif_data: &ExifData) -> Result<(), String> {
    let mut metadata =
        Metadata::new_from_path(path).map_err(|e| format!("Failed to read metadata: {}", e))?;

    // 撮影日時を書き込み
    if let (Some(date), Some(time)) = (&exif_data.date, &exif_data.time) {
        let datetime_str = format!(
            "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
            date.year(),
            date.month(),
            date.day(),
            time.hour(),
            time.minute(),
            time.second()
        );
        metadata.set_tag(ExifTag::DateTimeOriginal(datetime_str));
    }

    // GPS座標を書き込み
    if let Some(lat) = &exif_data.latitude {
        let (d, m, s) = decimal_to_dms(lat.degrees);
        metadata.set_tag(ExifTag::GPSLatitude(vec![
            uR64 { nominator: d as u32, denominator: 1 },
            uR64 { nominator: m as u32, denominator: 1 },
            uR64 { nominator: (s * 10000.0) as u32, denominator: 10000 },
        ]));
        metadata.set_tag(ExifTag::GPSLatitudeRef(if lat.is_positive {
            "N".to_string()
        } else {
            "S".to_string()
        }));
    }

    if let Some(lon) = &exif_data.longitude {
        let (d, m, s) = decimal_to_dms(lon.degrees);
        metadata.set_tag(ExifTag::GPSLongitude(vec![
            uR64 { nominator: d as u32, denominator: 1 },
            uR64 { nominator: m as u32, denominator: 1 },
            uR64 { nominator: (s * 10000.0) as u32, denominator: 10000 },
        ]));
        metadata.set_tag(ExifTag::GPSLongitudeRef(if lon.is_positive {
            "E".to_string()
        } else {
            "W".to_string()
        }));
    }

    metadata
        .write_to_file(path)
        .map_err(|e| format!("Failed to write EXIF: {}", e))
}

fn decimal_to_dms(decimal: f64) -> (f64, f64, f64) {
    let d = decimal.trunc();
    let m_full = (decimal - d) * 60.0;
    let m = m_full.trunc();
    let s = (m_full - m) * 60.0;
    (d.abs(), m.abs(), s.abs())
}
