use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::prelude::*;
use diesel_schemas::schema::configuration_note;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigNote {
    LimiteNoteAjournee,
    NbMatiereMaxComponse,
}

impl ConfigNote {
    pub fn config_id(self) -> String {
        match self {
            Self::LimiteNoteAjournee => "CONF1".into(),
            Self::NbMatiereMaxComponse => "CONF2".into(),
        }
    }
    pub fn default_value(self) -> BigDecimal {
        match self {
            Self::LimiteNoteAjournee => 6.into(),
            Self::NbMatiereMaxComponse => 2.into(),
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
    pub fn limite_note_ajournee(con: &mut PgConnection) -> BigDecimal {
        Self::LimiteNoteAjournee.get(con)
    }
    pub fn nb_matiere_max_componse(con: &mut PgConnection) -> u32 {
        Self::NbMatiereMaxComponse.get(con).to_u32().unwrap_or(2)
    }
}
