use std::{cmp::Ordering, collections::HashMap};

use bigdecimal::ToPrimitive;
use diesel::{prelude::*, QueryResult};
use diesel_schemas::view::v_etudiant_matiere_note;
use protos_commons::{ReleveNote, ReleveNoteStatus, ReleveNoteUnit, ReleveNoteUnitStatus};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::models::view::VEtudiantMatiereNote;

use super::{
    config_note::{type_calcul::TypeCalculNote, ConfigNote},
    sem_mat::SemestreMatieres,
};

use crate::models::table::Matiere;

#[derive(Debug, Clone)]
pub struct GetReleveNote {
    pub etudiant: String,
    pub semestre: String,
}

#[derive(Debug, Clone, Default)]
pub struct EtudiantNoteUnit {
    pub matiere: String,
    pub note: f64,
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
        let type_calcul = ConfigNote::type_calcul_note(con);
        let vnotes = self.get_v_notes(con)?;
        let matieres = self.get_semestre_matiere(con)?;
        let mut notes: EtudiantNotes = (&matieres).into();
        notes.seed(&vnotes, type_calcul);
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
        let notes = self.get_notes(con)?;
        let notes_vec_unique = notes.into_unique();
        let has_ajournee = notes_vec_unique.iter().any(|m| m.note < conf_ajournee);
        let much_componsed = notes_vec_unique
            .iter()
            .filter(|m| conf_ajournee <= m.note && m.note <= moyenne_generale)
            .count();
        let moyenne = notes.moyenne();
        let mut r_note_units: Vec<ReleveNoteUnit> = Vec::new();
        for note in notes_vec_unique {
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

fn unique_from_type_calcule(v_notes: &[&VEtudiantMatiereNote], type_calcul: TypeCalculNote) -> f64 {
    match type_calcul {
        TypeCalculNote::Max => v_notes
            .iter()
            .max_by(|a, b| a.valeur.cmp(&b.valeur))
            .and_then(|note| note.valeur.to_f64())
            .unwrap_or_default(),
        TypeCalculNote::Moyenne => {
            let count = v_notes.len();
            v_notes
                .iter()
                .flat_map(|note| note.valeur.to_f64())
                .sum::<f64>()
                / (count as f64)
        }
    }
}

impl EtudiantNotes {
    pub fn seed(&mut self, v_notes: &[VEtudiantMatiereNote], type_calcul: TypeCalculNote) {
        let folded_notes = v_notes.iter().fold(
            HashMap::<String, Vec<&VEtudiantMatiereNote>>::new(),
            |mut agg, note| {
                agg.entry(note.matiere.clone()).or_default().push(note);
                agg
            },
        );
        self.0.iter_mut().for_each(|note| match note {
            EtudiantNote::Unique(e) => {
                if let Some(fnote) = folded_notes.get(&e.matiere) {
                    e.note = unique_from_type_calcule(fnote, type_calcul);
                }
            }
            EtudiantNote::Choix(es) => {
                es.iter_mut().for_each(|e| {
                    if let Some(fnote) = folded_notes.get(&e.matiere) {
                        e.note = unique_from_type_calcule(fnote, type_calcul);
                    }
                });
            }
        });
    }
    pub fn moyenne(&self) -> f64 {
        let units = self.into_unique();
        if units.is_empty() {
            0f64
        } else {
            units.iter().map(|u| u.note).sum::<f64>() / (units.len() as f64)
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
        con.test_transaction(|con| {
            seed(con)?;
            let data = GetReleveNote {
                etudiant: "ETU001844".into(),
                semestre: "S5".into(),
            }
            .get_notes(con)?;
            assert_eq!(data.0.len(), 6);
            println!("{}", data.moyenne());
            Ok::<_, anyhow::Error>(())
        });
        Ok(())
    }
    #[test]
    fn test_releve() -> anyhow::Result<()> {
        let pool = crate::etablish_connection();
        let mut con = pool.get()?;
        con.test_transaction(|con| {
            seed(con)?;
            let data = GetReleveNote {
                etudiant: "ETU001844".into(),
                semestre: "S5".into(),
            }
            .get(con)?;
            assert_eq!(data.status, Into::<i32>::into(ReleveNoteStatus::SAjournee));
            println!("{}", data.credits);
            Ok::<_, anyhow::Error>(())
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests_s4_1 {
    use std::{fs::File, io::BufReader, path::Path};

    use csv::Reader;
    use diesel::{Connection, PgConnection};
    use itu_csv_import::CSVNote;

    use crate::modules::{
        config_note::{type_calcul::TypeCalculNote, ConfigNote},
        etudiant_note::GetReleveNote,
    };

    fn import<P: AsRef<Path>>(path: P, con: &mut PgConnection) -> anyhow::Result<()> {
        let notes = CSVNote::read(Reader::from_reader(BufReader::new(File::open(path)?)));
        CSVNote::inserts(notes, con)?;
        Ok(())
    }
    fn test_transaction<P, F, T>(path: P, transaction: F) -> anyhow::Result<T>
    where
        P: AsRef<Path>,
        F: FnOnce(&mut PgConnection) -> Result<T, anyhow::Error>,
    {
        let pool = crate::etablish_connection();
        let mut con = pool.get()?;
        Ok(con.test_transaction(|con| {
            import(path, con)?;
            transaction(con)
        }))
    }
    #[test]
    fn test_data_nasa() -> anyhow::Result<()> {
        test_transaction("./data/1 - test donne - moyenne note.csv", |con| {
            let data = GetReleveNote {
                etudiant: "ETU002454".into(),
                semestre: "S4".into(),
            }
            .get_notes(con)?;
            println!("{}", data.moyenne());
            assert_eq!(data.moyenne() as u32, 10);
            Ok(())
        })?;
        Ok(())
    }
    #[test]
    fn test_data_im_nabi() -> anyhow::Result<()> {
        test_transaction("./data/2 - test donne - moyenne note.csv", |con| {
            let data = GetReleveNote {
                etudiant: "ETU002455".into(),
                semestre: "S4".into(),
            }
            .get_notes(con)?;
            println!("{}", data.moyenne());
            assert_eq!(data.moyenne() as u32, 11);
            assert!(data.moyenne() < 11.5f64);
            assert!(data.moyenne() > 11.3f64);
            Ok(())
        })?;

        Ok(())
    }
    #[test]
    fn test_data_ikor_kanto() -> anyhow::Result<()> {
        test_transaction("./data/3 - test donne - moyenne note.csv", |con| {
            let data = GetReleveNote {
                etudiant: "ETU002456".into(),
                semestre: "S4".into(),
            }
            .get_notes(con)?;
            // println!("{:#?}", data);
            println!("{}", data.moyenne());
            assert_eq!(data.moyenne() as u32, 8);
            assert!(data.moyenne() < 8.9f64);
            assert!(data.moyenne() > 8.7f64);
            Ok(())
        })?;
        Ok(())
    }
    #[test]
    fn test_data_yuki_kanamaru() -> anyhow::Result<()> {
        test_transaction("./data/4 - test donne - moyenne note.csv", |con| {
            let getter = GetReleveNote {
                etudiant: "ETU002457".into(),
                semestre: "S4".into(),
            };

            {
                let data = getter.get_notes(con)?;
                // println!("{:#?}", data);
                println!("{}", data.moyenne());
                assert_eq!(data.moyenne() as u32, 16);
                assert!(data.moyenne() < 16.9f64);
                assert!(data.moyenne() > 16.7f64);
            }
            {
                println!("setting type calcul note");
                ConfigNote::TypeCalculNote
                    .upsert(TypeCalculNote::Moyenne.to_value().into(), con)?;
                println!("setted");
                let data = getter.get_notes(con)?;
                // println!("{:#?}", data);
                println!("{}", data.moyenne());
                assert_eq!(data.moyenne() as u32, 12);
                assert!(data.moyenne() < 12.5f64);
                assert!(data.moyenne() > 12.4f64);
            }
            Ok(())
        })
    }
}
