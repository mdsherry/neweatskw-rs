mod args;
mod facility;
mod fetch;
mod tweet;
mod update;

use anyhow::Error;
use args::Args;
use clap::Parser;
use rusqlite::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    match args {
        Args::Update(update) => update::update(update)?,
        Args::Fetch(fetch) => fetch::fetch(fetch).await?,
        Args::Tweet(tweet) => tweet::tweet(tweet).await?,
        Args::SanitizeDb(args) => sanitize_db(args)?,
    };
    Ok(())
}

fn sanitize_db(args: args::SanitizeDb) -> Result<(), Error> {
    let db = Connection::open(&args.database)?;
    db.execute("DELETE FROM settings WHERE key = 'twitter.access_key'", [])?;
    db.execute(
        "DELETE FROM settings WHERE key = 'twitter.access_secret'",
        [],
    )?;
    Ok(())
}
