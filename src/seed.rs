use diesel::{insert_into, prelude::*, PgConnection, QueryResult};

use crate::models::table::Semestre;

pub fn seed_semestre(con: &mut PgConnection) -> QueryResult<()> {
    use diesel_schemas::schema::semestre::dsl::*;
    let sems = vec![
        Semestre {
            id_sem: "S1".into(),
        },
        Semestre {
            id_sem: "S2".into(),
        },
        Semestre {
            id_sem: "S3".into(),
        },
        Semestre {
            id_sem: "S4".into(),
        },
        Semestre {
            id_sem: "S5".into(),
        },
        Semestre {
            id_sem: "S6".into(),
        },
    ];
    insert_into(semestre).values(sems).execute(con)?;
    Ok(())
}
