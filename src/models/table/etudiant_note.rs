use std::cmp::Ordering;

use bigdecimal::ToPrimitive;
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

#[derive(Debug, Clone, Default)]
pub struct EtudiantNoteUnit {
    matiere: String,
    note: f64,
}

#[derive(Debug, Clone)]
pub enum EtudiantNote {
    Unique(EtudiantNoteUnit),
    Choix(Vec<EtudiantNoteUnit>),
}

impl EtudiantNote {
    pub fn unit(&self) -> Option<EtudiantNoteUnit> {
        match self {
            EtudiantNote::Unique(u) => Some(u.clone()),
            EtudiantNote::Choix(c) => c
                .iter()
                .max_by(|a, b| a.note.partial_cmp(&b.note).unwrap_or(Ordering::Equal))
                .cloned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EtudiantNotes(pub Vec<EtudiantNote>);

fn now() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(now.date(), now.time())
}

impl GetReleveNote {
    pub fn get_semestre_matiere(&self, con: &mut PgConnection) -> QueryResult<SemestreMatieres> {
        SemestreMatieres::from_semestres(con, self.semestre.clone())
    }
    pub fn get_v_notes(&self, con: &mut PgConnection) -> QueryResult<Vec<VEtudiantMatiereNote>> {
        use self::v_etudiant_matiere_note::dsl::*;
        v_etudiant_matiere_note
            .filter(etu.eq(&self.etudiant))
            .filter(semestre.eq(&self.semestre))
            .filter(submission.le(now()))
            .select(VEtudiantMatiereNote::as_select())
            .load(con)
    }
    pub fn get(&self, con: &mut PgConnection) -> QueryResult<EtudiantNotes> {
        let vnotes = self.get_v_notes(con)?;
        let matieres = self.get_semestre_matiere(con)?;
        let mut notes: EtudiantNotes = (&matieres).into();
        notes.seed(&vnotes);
        Ok(notes)
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
                        e.note = value.valeur.to_f64().unwrap_or_default();
                    }
                    EtudiantNote::Choix(e) => {
                        if let Some(v) = e.iter_mut().find(|e| e.matiere == value.matiere) {
                            v.note = value.valeur.to_f64().unwrap_or_default();
                        }
                    }
                }
            }
        });
    }
    pub fn moyenne(&self) -> f64 {
        let units = self.0.iter().flat_map(|m| m.unit()).collect::<Vec<_>>();
        if units.is_empty() {
            0f64
        } else {
            units.iter().map(|u| u.note).sum::<f64>() / units.len() as f64
        }
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
                            note: 0f64,
                        })
                    }
                    super::sem_mat::SemestreMatiere::Choix(mats) => EtudiantNote::Choix(
                        mats.iter()
                            .map(|mat| EtudiantNoteUnit {
                                matiere: mat.id_matiere.clone(),
                                note: 0f64,
                            })
                            .collect(),
                    ),
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use diesel::{insert_into, prelude::*};
    use time::Date;
    use uuid::Uuid;

    use crate::models::table::{Etudiant, Note, Promotion};
    use diesel_schemas::schema::{etudiant, note, promotion};

    use super::{now, GetReleveNote};

    fn seed(con: &mut PgConnection) -> QueryResult<()> {
        let prom = Promotion {
            id_promotion: "P15".into(),
            nom: None,
        };
        let etudiant_ = Etudiant {
            etu: "ETU001844".into(),
            nom: "RAMANANTSALAMA".into(),
            prenom: "Dirk Tony".into(),
            date_naissance: Date::from_calendar_date(2006, time::Month::April, 6).unwrap(),
            promotion: prom.id_promotion.clone(),
            genre: 1,
        };
        let notes = vec![
            Note {
                id_note: Uuid::new_v4(),
                etudiant: etudiant_.etu.clone(),
                matiere: "INF304".into(),
                submission: now(),
                valeur: BigDecimal::from_f64(3.9).unwrap_or_default(),
            },
            Note {
                id_note: Uuid::new_v4(),
                etudiant: etudiant_.etu.clone(),
                matiere: "INF301".into(),
                submission: now(),
                valeur: BigDecimal::from_f64(10.0).unwrap_or_default(),
            },
            Note {
                id_note: Uuid::new_v4(),
                etudiant: etudiant_.etu.clone(),
                matiere: "ORG303".into(),
                submission: now(),
                valeur: BigDecimal::from_f64(12.82).unwrap_or_default(),
            },
            Note {
                id_note: Uuid::new_v4(),
                etudiant: etudiant_.etu.clone(),
                matiere: "INF307".into(),
                submission: now(),
                valeur: BigDecimal::from_f64(10f64).unwrap_or_default(),
            },
        ];
        insert_into(promotion::dsl::promotion)
            .values(prom)
            .execute(con)?;
        insert_into(etudiant::dsl::etudiant)
            .values(etudiant_)
            .execute(con)?;
        insert_into(note::dsl::note).values(notes).execute(con)?;
        Ok(())
    }
    #[test]
    fn test_data() -> anyhow::Result<()> {
        let pool = crate::etablish_connection();
        let mut con = pool.get()?;
        let _ = con.transaction(|con| seed(con));
        let data = GetReleveNote {
            etudiant: "ETU001844".into(),
            semestre: "S5".into(),
        }
        .get(&mut con)?;
        assert_eq!(data.0.len(), 6);
        println!("{}", data.moyenne());
        Ok(())
    }
}
