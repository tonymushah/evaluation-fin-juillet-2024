use diesel::{PgConnection, QueryResult};

use super::etudiant_note::get_etudiant_notes;

pub fn get_etudiant_moyenne(etu: &str, con: &mut PgConnection) -> QueryResult<f64> {
    let notes = get_etudiant_notes(etu, con)?;
    let notes_len = notes.len() as f64;
    Ok(notes.iter().map(|note| note.moyenne).sum::<f64>() / notes_len)
}
