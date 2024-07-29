use bigdecimal::BigDecimal;
use diesel::{prelude::*, QueryResult};
use diesel_schemas::view::v_etudiant_matiere_note;
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::models::view::VEtudiantMatiereNote;

use super::sem_mat::SemestreMatieres;

#[derive(Debug, Clone)]
pub struct GetReleveNote {
    pub etudiant: String,
    pub semestre: String,
}

#[derive(Debug, Clone)]
pub struct EtudiantNoteUnit {
    matiere: String,
    note: BigDecimal,
}

#[derive(Debug, Clone)]
pub enum EtudiantNote {
    Unique(EtudiantNoteUnit),
    Choix(Vec<EtudiantNoteUnit>),
}

#[derive(Debug, Clone)]
pub struct EtudiantNotes(pub Vec<EtudiantNote>);

impl GetReleveNote {
    pub fn get_semestre_matiere(&self, con: &mut PgConnection) -> QueryResult<SemestreMatieres> {
        SemestreMatieres::from_semestres(con, self.semestre.clone())
    }
    pub fn get_v_notes(&self, con: &mut PgConnection) -> QueryResult<Vec<VEtudiantMatiereNote>> {
        use self::v_etudiant_matiere_note::dsl::*;
        v_etudiant_matiere_note
            .filter(etu.eq(&self.etudiant))
            .filter(semestre.eq(&self.semestre))
            .filter(submission.le({
                let now = OffsetDateTime::now_utc();
                PrimitiveDateTime::new(now.date(), now.time())
            }))
            .select(VEtudiantMatiereNote::as_select())
            .load(con)
    }
}

impl EtudiantNotes {
    pub fn seed(&mut self, v_notes: &[VEtudiantMatiereNote]) {
        self.0.iter_mut().for_each(|note| {
            let notes = v_notes
                .iter()
                .filter(|n| {
                    let mat = match note {
                        EtudiantNote::Unique(e) => e.matiere.clone(),
                        EtudiantNote::Choix(e) => e
                            .iter()
                            .find(|e| e.matiere == n.matiere)
                            .map(|e| e.matiere.clone())
                            .unwrap_or_default(),
                    };
                    n.matiere == mat
                })
                .cloned()
                .collect::<Vec<VEtudiantMatiereNote>>();
            if let Some(value) = notes.iter().max_by(|a, b| a.submission.cmp(&b.submission)) {
                match note {
                    EtudiantNote::Unique(e) => {
                        e.note = value.valeur.clone();
                    }
                    EtudiantNote::Choix(e) => {
                        if let Some(v) = e.iter_mut().find(|e| e.matiere == value.matiere) {
                            v.note = value.valeur.clone();
                        }
                    }
                }
            }
        });
    }
}

impl From<&SemestreMatieres> for EtudiantNotes {
    fn from(value: &SemestreMatieres) -> Self {
        Self(
            value
                .0
                .iter()
                .map(|mat| match mat {
                    super::sem_mat::SemestreMatiere::Unique(mat) => {
                        EtudiantNote::Unique(EtudiantNoteUnit {
                            matiere: mat.id_matiere.clone(),
                            note: BigDecimal::default(),
                        })
                    }
                    super::sem_mat::SemestreMatiere::Choix(mats) => EtudiantNote::Choix(
                        mats.iter()
                            .map(|mat| EtudiantNoteUnit {
                                matiere: mat.id_matiere.clone(),
                                note: BigDecimal::default(),
                            })
                            .collect(),
                    ),
                })
                .collect(),
        )
    }
}
