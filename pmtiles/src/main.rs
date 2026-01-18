use std::io::{self};

mod binaries;
mod pmtiles;
mod protobufs;
mod tileid;

fn main() -> io::Result<()> {
    let file_path = "/mnt/f/GIS/GSI/optimal_bvmap-v1.pmtiles";
    println!("{file_path}");

    let pmtiles = pmtiles::PMTiles::open(file_path)?;
    pmtiles.header.print_info();
    //pmtiles.metadata.print_info();
    pmtiles.root_directory.entries.iter().for_each(|entry| {
        println!("{}", entry);
    });

    pmtiles.get(16, 58166, 25820);
    //pmtiles.get(16, 55234, 27904);


    Ok(())
}

