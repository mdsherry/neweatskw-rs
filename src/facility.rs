use anyhow::Error;
use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};
use serde::Deserialize;

use std::path::Path;
#[derive(Debug, Deserialize)]
pub struct Facility {
    #[serde(rename = "SUBCATEGORY")]
    pub typ: String,
    #[serde(rename = "CITY")]
    pub city: String,
    #[serde(rename = "ADDR")]
    pub address: String,
    #[serde(rename = "FACILITYID")]
    pub id: String,
    #[serde(rename = "BUSINESS_NAME")]
    pub name: String,
}

impl Facility {
    pub fn normalize(self) -> Self {
        Facility {
            city: self.city.to_uppercase(),
            id: self.id.to_uppercase(),
            ..self
        }
    }

    pub fn is_restaurant(&self) -> bool {
        let typ = &self.typ;
        typ.contains("Restaurant")
            || typ.contains("Food Take Out")
            || typ.contains("Baked Goods - Retail")
            || typ.contains("Ice Cream / Yogurt Vendor")
    }

    pub fn is_in_town(&self) -> bool {
        self.city == "WATERLOO" || self.city == "KITCHENER" || self.city == "ST.+JACOBS"
    }
}

fn from_both<T>(r: Result<T, T>) -> T {
    match r {
        Ok(v) => v,
        Err(v) => v,
    }
}

pub fn get_facilities(p: impl AsRef<Path>) -> Result<Vec<Facility>, Error> {
    let mut rdr = csv::Reader::from_path(p)?;
    let mut rv = vec![];
    for result in rdr.byte_records() {
        let record = result?;
        let id = from_both(
            ISO_8859_1
                .decode(&record[0], DecoderTrap::Replace)
                .map_err(|e| e.into_owned()),
        );
        let name = from_both(
            ISO_8859_1
                .decode(&record[1], DecoderTrap::Replace)
                .map_err(|e| e.into_owned()),
        );
        let address = from_both(
            ISO_8859_1
                .decode(&record[2], DecoderTrap::Replace)
                .map_err(|e| e.into_owned()),
        );
        let city = from_both(
            ISO_8859_1
                .decode(&record[3], DecoderTrap::Replace)
                .map_err(|e| e.into_owned()),
        );
        let typ = from_both(
            ISO_8859_1
                .decode(&record[5], DecoderTrap::Replace)
                .map_err(|e| e.into_owned()),
        );
        let facility = Facility {
            id,
            name,
            address,
            city,
            typ,
        };
        rv.push(facility);
    }
    Ok(rv)
}
