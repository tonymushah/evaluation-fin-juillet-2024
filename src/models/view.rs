use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_schemas::view::*;
use time::Date;
use time::PrimitiveDateTime;
use uuid::Uuid;

// VEtudiantNote struct
#[derive(Queryable, Selectable)]
#[diesel(table_name = v_etudiant_note)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VEtudiantNote {
    pub etu: String,
    pub etu_nom: String,
    pub etu_prenom: String,
    pub etu_dtn: Date,
    pub etu_prom: String,
    pub etu_genre: i32,
    pub id_note: Uuid,
    pub matiere: String,
    pub submission: PrimitiveDateTime,
    pub valeur: BigDecimal,
}

// VMatiereNote struct
#[derive(Queryable, Selectable)]
#[diesel(table_name = v_matiere_note)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VMatiereNote {
    pub id_note: Uuid,
    pub etudiant: String,
    pub matiere: String,
    pub submission: PrimitiveDateTime,
    pub valeur: BigDecimal,
    pub nom: String,
    pub semestre: String,
    pub credits: i32,
    pub optionel: bool,
}

// VEtudiantMatiereNote struct
#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = v_etudiant_matiere_note)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VEtudiantMatiereNote {
    pub etu: String,
    pub etu_nom: String,
    pub etu_prenom: String,
    pub etu_dtn: Date,
    pub etu_prom: String,
    pub etu_genre: i32,
    pub id_note: Uuid,
    pub matiere: String,
    pub submission: PrimitiveDateTime,
    pub valeur: BigDecimal,
    pub matiere_nom: String,
    pub semestre: String,
    pub matiere_credits: i32,
    pub matiere_optionel: bool,
}
