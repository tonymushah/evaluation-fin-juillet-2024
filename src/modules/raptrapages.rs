use proto_client::Ratrapage;
use protos_commons::{ReleveNote, ReleveNoteUnitStatus};

const RATRAPAGE_UNIT_MONTANT: f64 = 25000f64;

pub trait ExtractRatrapage {
    fn extract_ratrapage(&self) -> Vec<Ratrapage>;
}

impl ExtractRatrapage for ReleveNote {
    fn extract_ratrapage(&self) -> Vec<Ratrapage> {
        let semestre = &self.semestre;
        self.notes.iter().fold(Vec::new(), |mut acc, note| {
            if note.status == (ReleveNoteUnitStatus::MAjournee as i32) {
                acc.push(Ratrapage {
                    semestre: semestre.clone(),
                    matiere: note
                        .matiere
                        .clone()
                        .map(|m| m.numero)
                        .unwrap_or(String::from("undefined??")),
                    note: note.valeur as f64,
                    montant: RATRAPAGE_UNIT_MONTANT,
                });
            }
            acc
        })
    }
}
