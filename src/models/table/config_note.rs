pub mod type_calcul;

use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::prelude::*;
use diesel_schemas::schema::configuration_note;
use type_calcul::TypeCalculNote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigNote {
    LimiteNoteAjournee,
    NbMatiereMaxComponse,
    TypeCalculNote,
}

impl ConfigNote {
    pub fn config_id(self) -> String {
        match self {
            Self::LimiteNoteAjournee => "CONF1".into(),
            Self::NbMatiereMaxComponse => "CONF2".into(),
            Self::TypeCalculNote => "CONF3".into(),
        }
    }
    pub fn default_value(self) -> BigDecimal {
        match self {
            Self::LimiteNoteAjournee => 6.into(),
            Self::NbMatiereMaxComponse => 2.into(),
            Self::TypeCalculNote => 1.into(),
        }
    }
    pub fn get_from_db(self, con: &mut PgConnection) -> QueryResult<BigDecimal> {
        use self::configuration_note::dsl::*;
        configuration_note
            .filter(code.eq(self.config_id()))
            .select(valeur)
            .get_result(con)
    }
    pub fn get(self, con: &mut PgConnection) -> BigDecimal {
        self.get_from_db(con).unwrap_or(self.default_value())
    }
    pub fn limite_note_ajournee(con: &mut PgConnection) -> f64 {
        Self::LimiteNoteAjournee.get(con).to_f64().unwrap_or(6f64)
    }
    pub fn nb_matiere_max_componse(con: &mut PgConnection) -> u32 {
        Self::NbMatiereMaxComponse.get(con).to_u32().unwrap_or(2)
    }
    pub fn type_calcul_note(con: &mut PgConnection) -> TypeCalculNote {
        Self::TypeCalculNote
            .get(con)
            .to_u8()
            .and_then(|value| value.try_into().ok())
            .unwrap_or_default()
    }
    pub fn moyenne_generale(_con: &mut PgConnection) -> f64 {
        10f64
    }
    pub fn upsert_type_config_note(
        key: String,
        value: BigDecimal,
        con: &mut PgConnection,
    ) -> QueryResult<()> {
        use self::configuration_note::dsl::*;
        diesel::insert_into(configuration_note)
            .values((code.eq(key), valeur.eq(&value)))
            .on_conflict(code)
            .do_update()
            .set(valeur.eq(&value))
            .execute(con)?;
        Ok(())
    }
}
