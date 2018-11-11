use chrono::prelude::*;
use failure::Error;
use rusqlite::{types::ToSql, Connection, NO_PARAMS};

use facility::{Facility, get_facilities};
use args::Update;

fn add_to_queue(db: &Connection, facility: Facility) -> Result<(), Error> {
    db.execute("INSERT INTO queue (facilities_id) VALUES (?)", &[&facility.id])?;
    Ok(())
}

fn get_recent(db: &Connection, days: i32) -> Result<Vec<Facility>, Error> {
    let mut stmt = db.prepare(
        &format!("SELECT id, name, address, city FROM facilities WHERE creation >= date('now', '-{} days')", days)
    )?;
    let foo = stmt.query_map(NO_PARAMS, |row| Facility {
        id: row.get(0),
        name: row.get(1),
        address: row.get(2),
        city: row.get(3),
        typ: "".into(),
    })?;
    Ok(foo.collect::<Result<Vec<_>, _>>()?)
}

fn add_to_db(db: &Connection, facility: Facility, date: &NaiveDate) -> Result<(), Error> {
    if facility.is_restaurant() && facility.is_in_town() {
        let facility = facility.normalize();
        let mut query = db
            .prepare_cached("SELECT * FROM facilities WHERE id = ?1")
            .unwrap();
        if query.query(&[&facility.id])?.next().is_some() {
            println!("Updating time on {}", facility.name);
            let mut stmt = db
                .prepare_cached("UPDATE facilities SET lastupdate = ?1 WHERE id = ?2")
                .unwrap();
            stmt.execute(&[&date as &ToSql, &facility.id])?;
        } else {
            println!("Inserting {}", facility.name);
            db.prepare_cached(
                "INSERT INTO facilities (id, name, lastupdate, creation, address, city)
				VALUES (?1, ?2, ?3, ?3, ?4, ?5)",
            )?.execute(&[
                &facility.id as &ToSql,
                &facility.name,
                &date,
                &facility.address,
                &facility.city,
            ])?;
        }
    }
    Ok(())
}

pub fn update(args: Update) -> Result<(), Error> {
    
    let db = Connection::open(&args.database)?;

    let date = match args.date {
        Some(ref s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")?,
        None => Local::today().naive_local(),
    };

    if args.update {
        for facility in get_facilities(&args.datasource)? {
            add_to_db(&db, facility, &date)?;
        }
    }

    match args.get_recent {
        Some(days) => for result in get_recent(&db, days)? {
            println!("{:?}", result);
            if args.enqueue {
                add_to_queue(&db, result)?;
            }
        },
        None => ()
    };
    Ok(())
}