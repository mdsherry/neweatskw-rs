use crate::args;
use crate::facility::Facility;
use anyhow::Error;
use egg_mode::{KeyPair, Token};
use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::Connection;
use std::io::stdin;

const API_CONSUMER_KEY: &str = "0pu4enYO6ozCWk21udNsg";
const API_SECRET: &str = "N0QkdKOzgzypTExULlhKuze6ezmZ9w0VbEfcg8Tqgxg";

async fn authorize(con_token: KeyPair) -> Result<Token, Error> {
    let request_token = egg_mode::auth::request_token(&con_token, "oob").await?;
    println!("Go here: {}", egg_mode::auth::authorize_url(&request_token));
    let mut pin = String::new();
    println!("Enter PIN here: ");
    stdin().read_line(&mut pin)?;
    let (token, _, _) = egg_mode::auth::access_token(con_token, &request_token, pin).await?;
    Ok(token)
}

fn store_key(db: &Connection, token: &Token) -> Result<(), Error> {
    match token {
        egg_mode::Token::Access {
            access: ref access_token,
            ..
        } => {
            db.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('twitter.access_key', ?)",
                &[&access_token.key],
            )?;
            db.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('twitter.access_secret', ?);",
                &[&access_token.secret],
            )?;
        }
        _ => unimplemented!(),
    }
    Ok(())
}

fn get_key(db: &Connection) -> Result<(String, String), Error> {
    let key: String = db.query_row(
        "SELECT value FROM settings WHERE key = 'twitter.access_key'",
        [],
        |row| row.get(0),
    )?;
    let secret: String = db.query_row(
        "SELECT value FROM settings WHERE key = 'twitter.access_secret'",
        [],
        |row| row.get(0),
    )?;
    Ok((key, secret))
}

pub async fn tweet(args: args::Tweet) -> Result<(), Error> {
    let db = Connection::open(&args.database)?;
    let con_token = egg_mode::KeyPair::new(API_CONSUMER_KEY, API_SECRET);

    if args.authorize {
        let token = authorize(con_token).await?;
        store_key(&db, &token)?;
    } else {
        let (key, secret) = get_key(&db)?;
        let token = egg_mode::Token::Access {
            consumer: con_token,
            access: egg_mode::KeyPair::new(key, secret),
        };
        egg_mode::auth::verify_tokens(&token).await?;
        tweet_top_of_queue(&db, &token).await?;
    }
    Ok(())
}

async fn tweet_top_of_queue(db: &Connection, token: &egg_mode::Token) -> Result<(), Error> {
    let facility = db.query_row("SELECT id, name, address, city FROM facilities, queue WHERE facilities.ID = queue.facilities_id;", [], |row| Ok(Facility {
        id: row.get(0)?,
        name: row.get(1)?,
        address: row.get(2)?,
        city: row.get(3)?,
        typ: "".into(),
    }))?;
    let message = get_message(&facility);
    println!("{}", message);
    let draft = egg_mode::tweet::DraftTweet::new(message);
    draft.send(token).await?;
    delete_from_queue(db, &facility)?;
    Ok(())
}

fn delete_from_queue(db: &Connection, facility: &Facility) -> Result<(), Error> {
    db.execute(
        "DELETE FROM queue WHERE facilities_id = ?;",
        &[&facility.id],
    )?;
    Ok(())
}

fn get_message(facility: &Facility) -> String {
    static NKFM_NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"NKFM-(.*)$").unwrap());

    // This could be a bit more efficient with a Cow
    let resto_name = if let Some(caps) = NKFM_NAME_RE.captures(&facility.name) {
        format!("{} (Kitchener Famer's Market)", &caps[1])
    } else {
        facility.name.clone()
    };

    format!(
        "{name}: {address}, {city}",
        name = resto_name,
        address = facility.address,
        city = facility.city
    )
}
