use crate::args::Fetch;
use anyhow::{Context, Error};
use chrono::prelude::*;
use std::fs::{rename, File};
use std::io::Cursor;
use std::path::PathBuf;

pub async fn fetch(args: Fetch) -> Result<(), Error> {
    let response = reqwest::get(&args.url).await?;

    let bytes = response.bytes().await?;
    let input = Cursor::new(&bytes);
    let mut zipfile = zip::ZipArchive::new(input)?;
    let mut facilities = zipfile
        .by_name("Facilities_OpenData.csv")
        .context("Couldn't find file in ZIP file")?;
    let date = Local::today().naive_local();
    let mut src_path = PathBuf::new();
    src_path.push("testdata");
    src_path.push("Facilities_OpenData.csv");
    if src_path.exists() {
        let mut dst_path = PathBuf::new();
        dst_path.push("testdata");
        dst_path.push(format!(
            "Facilities_OpenData_{}.csv",
            date.format("%Y-%m-%d")
        ));
        rename(&src_path, &dst_path)
            .context("Failed to backup testdata/Facilities_OpenData.csv")?;
    }
    let mut f = File::create("testdata/Facilities_OpenData.csv")
        .context("Creating output facilities data CSV")?;
    std::io::copy(&mut facilities, &mut f)?;
    Ok(())
}
