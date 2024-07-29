use std::io::Read;

use bigdecimal::{BigDecimal, FromPrimitive};
use csv::Reader;
use serde::{Deserialize, Deserializer};
use time::Date;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Genre {
    #[serde(alias = "Masculin")]
    Masculin,
    #[serde(alias = "Féminin")]
    Feminin,
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
}

fn deserealize_decimale<'de, D>(de: D) -> Result<BigDecimal, D::Error>
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
