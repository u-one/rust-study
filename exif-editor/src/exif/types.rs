use chrono::{DateTime, Local, NaiveDate, NaiveTime};

#[derive(Clone, Debug, Default)]
pub struct GpsCoordinate {
    pub degrees: f64,
    pub is_positive: bool, // N/E = true, S/W = false
}

impl GpsCoordinate {
    pub fn new(degrees: f64, is_positive: bool) -> Self {
        Self { degrees, is_positive }
    }

    pub fn to_decimal(&self) -> f64 {
        if self.is_positive {
            self.degrees
        } else {
            -self.degrees
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExifData {
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub latitude: Option<GpsCoordinate>,
    pub longitude: Option<GpsCoordinate>,
}

#[derive(Clone, Debug)]
pub struct JpgFileEntry {
    pub path: std::path::PathBuf,
    pub filename: String,
    pub original_exif: ExifData,
    pub edited_exif: ExifData,
    pub is_modified: bool,
    pub file_created: Option<DateTime<Local>>,
    pub file_modified: Option<DateTime<Local>>,
}

impl JpgFileEntry {
    pub fn new(
        path: std::path::PathBuf,
        exif: ExifData,
        file_created: Option<DateTime<Local>>,
        file_modified: Option<DateTime<Local>>,
    ) -> Self {
        let filename = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        Self {
            path,
            filename,
            original_exif: exif.clone(),
            edited_exif: exif,
            is_modified: false,
            file_created,
            file_modified,
        }
    }

    pub fn reset(&mut self) {
        self.edited_exif = self.original_exif.clone();
        self.is_modified = false;
    }

    pub fn mark_modified(&mut self) {
        self.is_modified = self.edited_exif_differs();
    }

    fn edited_exif_differs(&self) -> bool {
        let orig = &self.original_exif;
        let edit = &self.edited_exif;

        orig.date != edit.date
            || orig.time != edit.time
            || !coords_equal(&orig.latitude, &edit.latitude)
            || !coords_equal(&orig.longitude, &edit.longitude)
    }
}

fn coords_equal(a: &Option<GpsCoordinate>, b: &Option<GpsCoordinate>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            (a.degrees - b.degrees).abs() < 0.000001 && a.is_positive == b.is_positive
        }
        _ => false,
    }
}
