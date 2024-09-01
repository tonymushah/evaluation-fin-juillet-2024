use diesel::{PgConnection, QueryResult};
use protos_commons::{EtudiantStatus, ReleveNoteStatus};

use super::etudiant_note::get_etudiant_notes;

pub fn get_etudiant_status(etu: &str, con: &mut PgConnection) -> QueryResult<EtudiantStatus> {
    let notes = get_etudiant_notes(etu, con)?;
    if notes
        .iter()
        .any(|note| note.status == (ReleveNoteStatus::SAjournee as i32))
    {
        Ok(EtudiantStatus::EAjournee)
    } else {
        Ok(EtudiantStatus::EAdmis)
    }
}
