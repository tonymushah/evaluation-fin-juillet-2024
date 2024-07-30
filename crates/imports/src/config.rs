use bigdecimal::BigDecimal;
use csv::Reader;
use diesel::prelude::*;
use diesel_schemas::schema::configuration_note;
use serde::Deserialize;

use crate::notes::deserealize_decimale;
use std::io::Read;

#[derive(Debug, Clone, Deserialize, Insertable, Selectable)]
#[diesel(table_name = configuration_note)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CSVConfigNote {
    pub code: String,
    pub config: String,
    #[serde(deserialize_with = "deserealize_decimale")]
    pub valeur: BigDecimal,
}

impl CSVConfigNote {
    pub fn read<R: Read>(mut reader: Reader<R>) -> Vec<Self> {
        reader.deserialize::<Self>().flatten().collect()
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader};

    use csv::Reader;

    use super::CSVConfigNote;
    #[test]
    fn read() -> anyhow::Result<()> {
        let buf_read = BufReader::new(File::open(
            "../../data/donnée import - juillet 2024 - configuration_note.csv",
        )?);
        let reader = Reader::from_reader(buf_read);
        for entry in CSVConfigNote::read(reader) {
            println!("{:#?}", entry);
        }
        Ok(())
    }
}
