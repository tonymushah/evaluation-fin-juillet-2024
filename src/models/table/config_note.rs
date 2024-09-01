use bigdecimal::{BigDecimal, ToPrimitive};
use diesel::prelude::*;
use diesel_schemas::schema::*;
use proto_admin::ConfigNote;

// Etudiant struct
#[derive(
    Queryable,
    Insertable,
    Selectable,
    AsChangeset,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Identifiable,
)]
#[diesel(primary_key(code))]
#[diesel(table_name = configuration_note)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConfigNoteEntry {
    pub code: String,
    pub config: String,
    pub valeur: BigDecimal,
}

impl From<ConfigNote> for ConfigNoteEntry {
    fn from(value: ConfigNote) -> Self {
        Self {
            code: value.code,
            config: value.name,
            valeur: value.value.try_into().unwrap_or_default(),
        }
    }
}

impl From<ConfigNoteEntry> for ConfigNote {
    fn from(value: ConfigNoteEntry) -> Self {
        Self {
            code: value.code,
            name: value.config,
            value: value.valeur.to_f64().unwrap_or_default(),
        }
    }
}
