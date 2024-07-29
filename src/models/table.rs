pub mod sem_mat;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::schema::*;
use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

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
