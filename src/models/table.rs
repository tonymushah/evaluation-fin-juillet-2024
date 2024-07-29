use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::schema::*;
use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

// Etudiant struct
#[derive(Queryable, Insertable)]
#[diesel(table_name = etudiant)]
pub struct Etudiant {
    pub etu: String,
    pub nom: String,
    pub prenom: String,
    pub date_naissance: Date,
    pub promotion: String,
    pub genre: i32,
}

// Matiere struct
#[derive(Queryable, Insertable)]
#[diesel(table_name = matiere)]
pub struct Matiere {
    pub id_matiere: String,
    pub credits: i32,
    pub semestre: i32,
    pub optionel: Option<bool>,
    pub nom: String,
}

// Note struct
#[derive(Queryable, Insertable)]
#[diesel(table_name = note)]
pub struct Note {
    pub id_note: Uuid,
    pub etudiant: String,
    pub matiere: String,
    pub submission: PrimitiveDateTime,
    pub valeur: BigDecimal,
}

// Promotion struct
#[derive(Queryable, Insertable)]
#[diesel(table_name = promotion)]
pub struct Promotion {
    pub id_promotion: String,
    pub nom: Option<String>,
}

// Semestre struct
#[derive(Queryable, Insertable)]
#[diesel(table_name = semestre)]
pub struct Semestre {
    pub id_sem: i32,
}
