mod protos {
    tonic::include_proto!("mg.tonymushah.evalfjuil24");
}

pub use protos::*;
use time::macros::format_description;

impl From<time::Date> for Date {
    fn from(value: time::Date) -> Self {
        Self {
            jour: value.day() as u32,
            mois: (value.month() as u8) as u32,
            annee: value.year() as u32,
        }
    }
}

impl TryFrom<Date> for time::Date {
    type Error = time::error::Parse;
    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let format = format_description!("[year]-[month]-[day]");
        time::Date::parse(
            &format!("{}-{}-{}", value.annee, value.mois, value.jour),
            format,
        )
    }
}

impl From<u8> for Genre {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::GMasculin,
            2 => Self::GFeminin,
            _ => Self::GAutre,
        }
    }
}
