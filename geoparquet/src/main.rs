use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::data_type::ByteArray;
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::error::Error;
use geoparquet::reader::{GeoParquetReaderBuilder};
use serde_json::Value;

fn main() -> Result<(), Box<dyn Error>> {

    read_with_parquet_rs()
    //read_with_geoparquet()
}

fn read_with_geoparquet() -> Result<(), Box<dyn Error>> {
    let path = "example.parquet";
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    let geoparquet_metadata = builder.geoparquet_metadata().unwrap().unwrap();
    println!("GeoParquet Metadata: {:?}", geoparquet_metadata);

    Ok(())
}

fn read_with_parquet_rs() -> Result<(), Box<dyn Error>> {
    let path = "example.parquet";
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let metadata = reader.metadata();
    let file_metadata = metadata.file_metadata();
    println!("File Metadata:");
    println!("version: {}", file_metadata.version());
    println!("Num Rows: {}", file_metadata.num_rows());
    println!("Created By: {:?}", file_metadata.created_by());
    println!("Schema:");
    println!("  Basic info: {:?}", file_metadata.schema().get_basic_info());
    println!("  Fields:");
    for field in file_metadata.schema().get_fields() {
        println!("    {:?}", field);
    }
    println!("Key Value Metadata:");
    file_metadata.key_value_metadata().unwrap().iter().for_each(|kv| {
        println!("  Key: {:?}, Value: {:?}", kv.key, kv.value);
    });
    let key_value_metadata = file_metadata.key_value_metadata().unwrap();
    // A GeoParquet file MUST include a geo key in the Parquet metadata
    let geo = key_value_metadata.iter()
        .find(|kv| kv.key == "geo").unwrap();
    // The value of this key MUST be a JSON-encoded UTF-8 string representing the file 
    // and column metadata that validates against the GeoParquet metadata schema.
    let geo_json = geo.value.as_ref().unwrap();
    println!("GeoParquet Geo Metadata: {}", geo_json);

    let value = serde_json::from_str::<Value>(geo_json);
    println!("GeoParquet Geo Metadata (parsed): {:?}", value);

    for i in 0..reader.num_row_groups() {
        let rg = reader.get_row_group(i)?;
        println!("Row Group {}: ", i);
        println!("  Columns:");
        for j in 0..rg.num_columns() {
            let col = rg.get_column_reader(j)?;
            println!("    Column Descriptor: {:?}", col.try_into());
        }
    }



    //metadata.row_groups().iter().enumerate().for_each(|(i, rg)| {
    //    println!("Row Group {}: ", i);
    //    println!("  Num Rows: {}", rg.num_rows());
    //    println!("  Total Byte Size: {}", rg.total_byte_size());
    //    println!("  Columns:");
    //    rg.columns().iter().for_each(|col| {
    //        println!("    Column Descriptor: {:?}", col.column_descr());
    //        println!("    Num values: {:?}", col.num_values());
    //    });
    //});

    //let iter = reader.get_row_iter(None)?;
    //for (_i, _record) in iter.enumerate() {
    //    //println!("Record {}: {:?}", i, record);
    //}
    Ok(())

}