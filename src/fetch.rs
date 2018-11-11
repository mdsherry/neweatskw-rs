use args::Fetch;
use chrono::prelude::*;
use failure::Error;
use reqwest;
use std::fs::{rename, File};
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use zip;

pub fn fetch(args: Fetch) -> Result<(), Error> {
    let mut response = reqwest::get(&args.url)?;
    let mut buf = vec![];
    response.read_to_end(&mut buf)?;
    let input = Cursor::new(&buf);
    let mut zipfile = zip::ZipArchive::new(input)?;
    let facilities = zipfile.by_name("Facilities_OpenData.csv")?;
    let date = Local::today().naive_local();
    {
        let mut src_path = PathBuf::new();
        src_path.push("testdata");
        src_path.push("Facilities_OpenData.csv");
        let mut dst_path = PathBuf::new();
        dst_path.push("testdata");
        dst_path.push(format!(
            "Facilities_OpenData_{}.csv",
            date.format("%Y-%m-%d")
        ));
        rename(&src_path, &dst_path)?;
    }
    let mut f = File::create("testdata/Facilities_OpenData.csv")?;
    read_into(facilities, &mut f)?;
    Ok(())
}

fn read_into(mut r: impl Read, w: &mut impl Write) -> std::io::Result<usize> {
    let mut total_bytes_read = 0;
    let mut buf = [0u8; 4096];
    loop {
        let bytes_read = r.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        total_bytes_read += bytes_read;
        w.write(&buf[..bytes_read])?;
    }
    Ok(total_bytes_read)
}
