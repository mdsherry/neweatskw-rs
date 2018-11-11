#[macro_use]
extern crate structopt;
extern crate chrono;
extern crate csv;
extern crate failure;
extern crate rusqlite;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate egg_mode;
extern crate zip;
extern crate reqwest;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate encoding;

mod args;
mod facility;
mod update;
mod fetch;
mod tweet;

use structopt::StructOpt;
use failure::Error;
use args::Args;
use rusqlite::{Connection, NO_PARAMS};

fn main() -> Result<(), Error> {
    let args = Args::from_args();
    match args {
        Args::Update(update) => update::update(update)?,
        Args::Fetch(fetch) => fetch::fetch(fetch)?,
        Args::Tweet(tweet) => tweet::tweet(tweet)?,
        Args::SanitizeDb(sanitize_db) => ::sanitize_db(sanitize_db)?
    };
    Ok(())
}


fn sanitize_db(args: args::SanitizeDb) -> Result<(), Error> {
    let db = Connection::open(&args.database)?;
    db.execute(
        "DELETE FROM settings WHERE key = 'twitter.access_key'",
        NO_PARAMS
    )?;
    db.execute(
        "DELETE FROM settings WHERE key = 'twitter.access_secret'",
        NO_PARAMS
    )?;
    Ok(())
}