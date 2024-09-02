use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use proto_admin::EtudiantRangEntry;

use crate::models::table::Etudiant;

use super::etudiant_note::GetReleveNote;

pub fn get_semestre_etudiant_rangs(
    semestre: &str,
    con: &mut PgConnection,
) -> QueryResult<Vec<EtudiantRangEntry>> {
    let etudiants = {
        use diesel_schemas::schema::etudiant::dsl::*;
        etudiant.select(Etudiant::as_select()).get_results(con)?
    };
    let mut list = etudiants
        .into_iter()
        .map(|etu| -> QueryResult<EtudiantRangEntry> {
            let releve = GetReleveNote {
                semestre: semestre.into(),
                etudiant: etu.etu.clone(),
            }
            .get(con)?;
            Ok(EtudiantRangEntry {
                etu: etu.etu,
                nom: etu.nom,
                prenom: etu.prenom,
                moyenne: releve.moyenne,
            })
        })
        .collect::<QueryResult<Vec<_>>>()?;
    list.sort_by(|a, b| b.moyenne.total_cmp(&a.moyenne));
    Ok(list)
}
