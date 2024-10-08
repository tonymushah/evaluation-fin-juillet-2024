pub mod config_note;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::schema::*;
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::modules::{
    etudiant_moyenne::get_etudiant_moyenne, etudiant_note::now,
    etudiant_status::get_etudiant_status,
};

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

impl Etudiant {
    pub fn into_proto(self, con: &mut PgConnection) -> QueryResult<protos_commons::Etudiant> {
        let etu = &self.etu;
        let age_dur = OffsetDateTime::now_utc().date() - self.date_naissance;
        let age = {
            let now = now().date();
            let next = now + age_dur;
            next.year() - now.year()
        } as u32;
        Ok(protos_commons::Etudiant {
            numero: self.etu.clone(),
            nom: self.nom,
            prenom: self.prenom,
            date_naissance: Some(self.date_naissance.into()),
            age,
            promotion: self.promotion,
            genre: (self.genre as u8).into(),
            moyenne: get_etudiant_moyenne(etu, con)?,
            status: get_etudiant_status(etu, con)? as i32,
        })
    }
}
