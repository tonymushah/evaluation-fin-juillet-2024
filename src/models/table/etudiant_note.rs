use std::cmp::Ordering;

use bigdecimal::ToPrimitive;
use diesel::{prelude::*, QueryResult};
use diesel_schemas::view::v_etudiant_matiere_note;
use protos_commons::{ReleveNote, ReleveNoteStatus, ReleveNoteUnit, ReleveNoteUnitStatus};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::models::view::VEtudiantMatiereNote;

use super::{config_note::ConfigNote, sem_mat::SemestreMatieres, Matiere};

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

pub fn now() -> PrimitiveDateTime {
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
    pub fn get_notes(&self, con: &mut PgConnection) -> QueryResult<EtudiantNotes> {
        let vnotes = self.get_v_notes(con)?;
        let matieres = self.get_semestre_matiere(con)?;
        let mut notes: EtudiantNotes = (&matieres).into();
        notes.seed(&vnotes);
        Ok(notes)
    }
    fn get_matiere(&self, mat: String, con: &mut PgConnection) -> QueryResult<Matiere> {
        use diesel_schemas::schema::matiere::dsl::*;
        matiere
            .filter(id_matiere.eq(mat))
            .select(Matiere::as_select())
            .get_result(con)
    }
    pub fn get(&self, con: &mut PgConnection) -> QueryResult<ReleveNote> {
        let conf_ajournee = ConfigNote::limite_note_ajournee(con);
        let conf_limit_comp = ConfigNote::nb_matiere_max_componse(con);
        let moyenne_generale = ConfigNote::moyenne_generale(con);
        let notes = self.get_notes(con)?.into_unique();
        let has_ajournee = notes.iter().any(|m| m.note < conf_ajournee);
        let much_componsed = notes
            .iter()
            .filter(|m| conf_ajournee <= m.note && m.note <= moyenne_generale)
            .count();
        let moyenne = notes.iter().map(|u| u.note).sum::<f64>() / notes.len() as f64;
        let mut r_note_units: Vec<ReleveNoteUnit> = Vec::new();
        for note in notes {
            let matiere = self.get_matiere(note.matiere.clone(), con)?;
            r_note_units.push(ReleveNoteUnit {
                matiere: Some(matiere.into()),
                valeur: note.note as f32,
                status: if note.note < conf_ajournee {
                    ReleveNoteUnitStatus::MAjournee.into()
                } else if conf_ajournee <= note.note && note.note < moyenne_generale {
                    if much_componsed > (conf_limit_comp as usize) {
                        ReleveNoteUnitStatus::MAjournee.into()
                    } else {
                        ReleveNoteUnitStatus::MCompensee.into()
                    }
                } else {
                    ReleveNoteUnitStatus::MValid.into()
                },
            });
        }
        Ok(ReleveNote {
            credits: r_note_units
                .iter()
                .flat_map(|note| {
                    if note.status == Into::<i32>::into(ReleveNoteUnitStatus::MAjournee) {
                        None
                    } else {
                        Some(note.matiere.as_ref()?.credits as u64)
                    }
                })
                .sum::<u64>(),
            semestre: self.semestre.clone(),
            status: if has_ajournee || much_componsed > (conf_limit_comp as usize) {
                ReleveNoteStatus::SAjournee.into()
            } else {
                ReleveNoteStatus::SValid.into()
            },
            moyenne,
            notes: r_note_units,
        })
    }
}

pub fn get_etudiant_notes(etudiant: &str, con: &mut PgConnection) -> QueryResult<Vec<ReleveNote>> {
    let sems: Vec<String> = {
        use diesel_schemas::schema::semestre::dsl::*;
        semestre.select(id_sem).get_results(con)?
    };
    sems.into_iter()
        .map(|semestre| {
            GetReleveNote {
                etudiant: etudiant.into(),
                semestre,
            }
            .get(con)
        })
        .collect()
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
        let units = self.into_unique();
        if units.is_empty() {
            0f64
        } else {
            units.iter().map(|u| u.note).sum::<f64>() / units.len() as f64
        }
    }
    pub fn into_unique(&self) -> Vec<EtudiantNoteUnit> {
        self.0.iter().flat_map(|m| m.unit()).collect::<Vec<_>>()
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
mod tests_s5 {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use diesel::{insert_into, prelude::*};
    use protos_commons::ReleveNoteStatus;
    use time::Date;
    use uuid::Uuid;

    use crate::models::table::{Etudiant, Note, Promotion};
    use diesel_schemas::schema::{etudiant, note, promotion};

    use super::{now, GetReleveNote};

    fn seed(con: &mut PgConnection) -> QueryResult<Vec<Note>> {
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
        insert_into(note::dsl::note).values(&notes).execute(con)?;
        Ok(notes)
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
        .get_notes(&mut con)?;
        assert_eq!(data.0.len(), 6);
        println!("{}", data.moyenne());
        Ok(())
    }
    #[test]
    fn test_releve() -> anyhow::Result<()> {
        let pool = crate::etablish_connection();
        let mut con = pool.get()?;
        let _ = con.transaction(|con| seed(con));
        let data = GetReleveNote {
            etudiant: "ETU001844".into(),
            semestre: "S5".into(),
        }
        .get(&mut con)?;
        assert_eq!(data.status, Into::<i32>::into(ReleveNoteStatus::SAjournee));
        println!("{}", data.credits);
        Ok(())
    }
}
