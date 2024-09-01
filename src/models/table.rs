pub mod config_note;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::schema::*;
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::modules::etudiant_note::now;

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
#[diesel(primary_key(etu))]
#[diesel(table_name = etudiant)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Etudiant {
    pub etu: String,
    pub nom: String,
    pub prenom: String,
    pub date_naissance: Date,
    pub promotion: String,
    pub genre: i32,
}

// Matiere struct
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
#[diesel(primary_key(id_matiere))]
#[diesel(table_name = matiere)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Matiere {
    pub id_matiere: String,
    pub credits: i32,
    pub semestre: String,
    pub optionel: Option<bool>,
    pub nom: String,
}

// Note struct
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
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = note)]
#[diesel(primary_key(id_note))]
pub struct Note {
    pub id_note: Uuid,
    pub etudiant: String,
    pub matiere: String,
    pub submission: PrimitiveDateTime,
    pub valeur: BigDecimal,
}

// Promotion struct
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
#[diesel(primary_key(id_promotion))]
#[diesel(table_name = promotion)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Promotion {
    pub id_promotion: String,
    pub nom: Option<String>,
}

// Semestre struct
#[derive(Queryable, Insertable, Selectable, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = semestre)]
pub struct Semestre {
    pub id_sem: String,
}

impl From<Matiere> for protos_commons::Matiere {
    fn from(value: Matiere) -> Self {
        Self {
            numero: value.id_matiere,
            nom: value.nom,
            credits: value.credits as u32,
        }
    }
}

impl From<Etudiant> for protos_commons::Etudiant {
    fn from(value: Etudiant) -> Self {
        let age_dur = OffsetDateTime::now_utc().date() - value.date_naissance;
        let age = {
            let now = now().date();
            let next = now + age_dur;
            next.year() - now.year()
        } as u32;
        Self {
            numero: value.etu,
            nom: value.nom,
            prenom: value.prenom,
            date_naissance: Some(value.date_naissance.into()),
            age,
            promotion: value.promotion,
            genre: (value.genre as u8).into(),
        }
    }
}
