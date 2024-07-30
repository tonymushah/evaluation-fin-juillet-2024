use super::CSVNote;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::schema::*;
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

pub fn collect_promotions<'a, I: Iterator<Item = &'a CSVNote>>(iter: I) -> Vec<String> {
    iter.fold(Vec::new(), |mut acc, note| {
        if !acc.contains(&note.prom) {
            acc.push(note.prom.clone());
        }
        acc
    })
}

// Etudiant struct
#[derive(Insertable)]
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

impl From<CSVNote> for Etudiant {
    fn from(value: CSVNote) -> Self {
        Self {
            etu: value.etu,
            nom: value.nom,
            prenom: value.prenom,
            date_naissance: value.naissance,
            promotion: value.prom,
            genre: {
                let u: u8 = value.genre.into();
                u as i32
            },
        }
    }
}

pub fn collect_etudiant<'a, I: Iterator<Item = &'a CSVNote>>(iter: I) -> Vec<Etudiant> {
    iter.fold(Vec::new(), |mut acc, note| {
        if !acc.iter().any(|et| et.etu == note.etu) {
            acc.push(note.clone().into());
        }
        acc
    })
}

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = note)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Note {
    pub id_note: Uuid,
    pub etudiant: String,
    pub matiere: String,
    pub submission: PrimitiveDateTime,
    pub valeur: BigDecimal,
}

impl From<CSVNote> for Note {
    fn from(value: CSVNote) -> Self {
        Self {
            id_note: Uuid::new_v4(),
            etudiant: value.etu,
            matiere: value.matiere,
            submission: {
                let now = OffsetDateTime::now_utc();
                PrimitiveDateTime::new(now.date(), now.time())
            },
            valeur: value.note,
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = promotion)]
pub struct Promotion {
    pub id_promotion: String,
    pub nom: Option<String>,
}
