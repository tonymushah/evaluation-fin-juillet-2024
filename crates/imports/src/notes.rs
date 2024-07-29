pub mod models;

use std::io::Read;

use bigdecimal::{BigDecimal, FromPrimitive};
use csv::Reader;
use diesel::{insert_into, PgConnection, QueryResult, RunQueryDsl};
use models::{collect_etudiant, collect_promotions, Note, Promotion};
use serde::{Deserialize, Deserializer};
use time::Date;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Genre {
    #[serde(alias = "Masculin")]
    Masculin,
    #[serde(alias = "Féminin")]
    Feminin,
}

#[derive(Clone, Copy, Debug)]
pub struct ParseGenreError(pub(crate) ());

impl std::fmt::Display for ParseGenreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input for parsing a int to Genre")
    }
}

impl std::error::Error for ParseGenreError {}

impl TryFrom<u8> for Genre {
    type Error = ParseGenreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Masculin),
            2 => Ok(Self::Feminin),
            _ => Err(ParseGenreError(())),
        }
    }
}

impl From<Genre> for u8 {
    fn from(value: Genre) -> Self {
        match value {
            Genre::Feminin => 2,
            Genre::Masculin => 1,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CSVNote {
    #[serde(alias = "NumETU")]
    pub etu: String,
    #[serde(alias = "Nom")]
    pub nom: String,
    #[serde(alias = "Prénom")]
    pub prenom: String,
    #[serde(alias = "Genre")]
    pub genre: Genre,
    #[serde(alias = "DateNaissance", deserialize_with = "super::deserealize_date")]
    pub naissance: Date,
    #[serde(alias = "Promotion")]
    pub prom: String,
    #[serde(alias = "CodeMatiere")]
    pub matiere: String,
    #[serde(alias = "Semestre")]
    pub sem: String,
    #[serde(alias = "Note", deserialize_with = "deserealize_decimale")]
    pub note: BigDecimal,
}

impl CSVNote {
    pub fn read<R: Read>(mut reader: Reader<R>) -> Vec<Self> {
        reader.deserialize::<Self>().flatten().collect()
    }
    pub fn inserts(values: Vec<Self>, con: &mut PgConnection) -> QueryResult<()> {
        let promotions = collect_promotions(values.iter());
        let etudiants = collect_etudiant(values.iter());
        {
            use diesel_schemas::schema::promotion::dsl::*;
            insert_into(promotion)
                .values(
                    promotions
                        .into_iter()
                        .map(|prom| Promotion {
                            id_promotion: prom,
                            nom: None,
                        })
                        .collect::<Vec<Promotion>>(),
                )
                .execute(con)?;
        }
        {
            use diesel_schemas::schema::etudiant::dsl::*;
            insert_into(etudiant).values(etudiants).execute(con)?;
        }
        {
            use diesel_schemas::schema::note::dsl::*;
            insert_into(note)
                .values(values.into_iter().map(|v| v.into()).collect::<Vec<Note>>())
                .execute(con)?;
        }
        Ok(())
    }
}

pub fn deserealize_decimale<'de, D>(de: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s: String = Deserialize::deserialize(de)?;
    s = s.replace(',', ".");
    //println!("{s}");
    BigDecimal::from_f64(s.parse::<f64>().map_err(serde::de::Error::custom)?)
        .ok_or(serde::de::Error::custom("cannot convert f64 to BigDecimal"))
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader};

    use csv::Reader;

    use super::CSVNote;
    #[test]
    fn read() -> anyhow::Result<()> {
        let buf_read = BufReader::new(File::open(
            "../../data/donnée import - juillet 2024 - note.csv",
        )?);
        let reader = Reader::from_reader(buf_read);
        for entry in CSVNote::read(reader) {
            println!("{:#?}", entry);
        }
        Ok(())
    }
}
