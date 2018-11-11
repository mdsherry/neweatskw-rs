use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Update {
    /// Update the database with new restaurants from CSV
    #[structopt(long = "update")]
    pub update: bool,

    /// File to obtain data from (default: testdata/Facilities_OpenData.csv)
    #[structopt(
        long = "datasource",
        parse(from_os_str),
        default_value = "testdata/Facilities_OpenData.csv"
    )]
    pub datasource: PathBuf,

    /// SQLite database file for storing updates
    #[structopt(
        long = "database",
        parse(from_os_str),
        default_value = "restaurants.db"
    )]
    pub database: PathBuf,

    /// Return informaiton on the N restaurants discovered in the last N days
    #[structopt(long = "getrecent")]
    pub get_recent: Option<i32>,

    /// For --getrecent, store the recent additions in the database.
    #[structopt(long = "enqueue")]
    pub enqueue: bool,

    /// The date of the update - format YYYY-MM-DD
    #[structopt(long = "date")]
    pub date: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct Fetch {
    /// File to obtain data from (default: testdata/Facilities_OpenData.csv)
    #[structopt(
        long = "datasource",
        parse(from_os_str),
        default_value = "testdata/Facilities_OpenData.csv"
    )]
    pub datasource: PathBuf,

    #[structopt(
        long = "url",
        default_value = "http://www.regionofwaterloo.ca/opendatadownloads/Inspections.zip"
    )]
    pub url: String,
}

#[derive(Debug, StructOpt)]
pub struct Tweet {
    /// Authorize this script to update your Twitter account.
    #[structopt(long="authorize")]
    pub authorize: bool,

    /// SQLite database file for storing updates
    #[structopt(
        long = "database",
        parse(from_os_str),
        default_value = "restaurants.db"
    )]
    pub database: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct SanitizeDb {
    /// SQLite database file for storing updates
    #[structopt(
        long = "database",
        parse(from_os_str),
        default_value = "restaurants.db"
    )]
    pub database: PathBuf,
}

#[derive(Debug, StructOpt)]
pub enum Args {
    #[structopt(name = "update")]
    Update(Update),
    #[structopt(name = "fetch")]
    Fetch(Fetch),
    #[structopt(name = "tweet")]
    Tweet(Tweet),
    #[structopt(name = "sanitizedb")]
    SanitizeDb(SanitizeDb)
}
