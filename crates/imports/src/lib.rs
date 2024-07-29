use serde::{Deserialize, Deserializer};
use time::{format_description, Date, PrimitiveDateTime, Time};

pub mod notes;

pub use notes::{CSVNote, Genre};

pub const IMPORT_DATE_TIME_FORMAT: &str = "[day]/[month]/[year] [hour]:[minute]:[second]";

pub const IMPORT_DATE_FORMAT: &str = "[day]/[month]/[year]";

pub const IMPORT_TIME_FORMAT: &str = "[hour]:[minute]:[second]";

pub fn deserealize_primitive_date_time<'de, D>(de: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
    // println!("{s}");
    let format =
        format_description::parse(IMPORT_DATE_TIME_FORMAT).map_err(serde::de::Error::custom)?;

    PrimitiveDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
}

pub fn deserealize_date<'de, D>(de: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
    //println!("{s}");
    let format = format_description::parse(IMPORT_DATE_FORMAT).map_err(serde::de::Error::custom)?;
    Date::parse(&s, &format).map_err(serde::de::Error::custom)
}

pub fn deserealize_time<'de, D>(de: D) -> Result<Time, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(de)?;
    //println!("{s}");
    let format = format_description::parse(IMPORT_TIME_FORMAT).map_err(serde::de::Error::custom)?;
    Time::parse(&s, &format).map_err(serde::de::Error::custom)
}
