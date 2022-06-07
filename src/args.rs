use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Update {
    /// Update the database with new restaurants from CSV
    #[clap(long = "update")]
    pub update: bool,

    /// File to obtain data from (default: testdata/Facilities_OpenData.csv)
    #[clap(
        long = "datasource",
        parse(from_os_str),
        default_value = "testdata/Facilities_OpenData.csv"
    )]
    pub datasource: PathBuf,

    /// SQLite database file for storing updates
    #[clap(
        long = "database",
        parse(from_os_str),
        default_value = "restaurants.db"
    )]
    pub database: PathBuf,

    /// Return informaiton on the N restaurants discovered in the last N days
    #[clap(long = "getrecent")]
    pub get_recent: Option<i32>,

    /// For --getrecent, store the recent additions in the database.
    #[clap(long = "enqueue")]
    pub enqueue: bool,

    /// The date of the update - format YYYY-MM-DD
    #[clap(long = "date")]
    pub date: Option<String>,
}

#[derive(Debug, Parser)]
pub struct Fetch {
    /// File to obtain data from (default: testdata/Facilities_OpenData.csv)
    #[clap(
        long = "datasource",
        parse(from_os_str),
        default_value = "testdata/Facilities_OpenData.csv"
    )]
    pub datasource: PathBuf,

    #[clap(
        long = "url",
        default_value = "http://www.regionofwaterloo.ca/opendatadownloads/Inspections.zip"
    )]
    pub url: String,
}

#[derive(Debug, Parser)]
pub struct Tweet {
    /// Authorize this script to update your Twitter account.
    #[clap(long = "authorize")]
    pub authorize: bool,

    /// SQLite database file for storing updates
    #[clap(
        long = "database",
        parse(from_os_str),
        default_value = "restaurants.db"
    )]
    pub database: PathBuf,
}

#[derive(Debug, Parser)]
pub struct SanitizeDb {
    /// SQLite database file for storing updates
    #[clap(
        long = "database",
        parse(from_os_str),
        default_value = "restaurants.db"
    )]
    pub database: PathBuf,
}

#[derive(Debug, Parser)]
pub enum Args {
    #[clap(name = "update")]
    Update(Update),
    #[clap(name = "fetch")]
    Fetch(Fetch),
    #[clap(name = "tweet")]
    Tweet(Tweet),
    #[clap(name = "sanitizedb")]
    SanitizeDb(SanitizeDb),
}
