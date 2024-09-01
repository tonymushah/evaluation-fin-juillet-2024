use std::collections::HashMap;

use diesel::{prelude::*, PgConnection, QueryResult};
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

pub fn get_etudiants_status(
    con: &mut PgConnection,
) -> QueryResult<HashMap<String, EtudiantStatus>> {
    let etus: Vec<String> = {
        use diesel_schemas::schema::etudiant::dsl::*;
        etudiant.select(etu).get_results(con)?
    };
    etus.into_iter()
        .map(|etu| -> QueryResult<_> { Ok((etu.clone(), get_etudiant_status(&etu, con)?)) })
        .collect()
}
