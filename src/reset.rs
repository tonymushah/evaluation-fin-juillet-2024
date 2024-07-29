use std::env;

use diesel::connection::SimpleConnection;

use crate::{seed::seed, DbPoolConnection};

pub fn reset_db_no_seed(con: &mut DbPoolConnection) -> crate::Result<()> {
    let user = env::var("DBUSER")?;
    let query = format!("SELECT truncate_tables('{user}');");
    if cfg!(debug_assertions) {
        println!("{query}");
    }
    con.batch_execute(query.as_str()).map_err(|e| {
        eprintln!("{:?}", e);
        e
    })?;
    Ok(())
}

pub fn reset_db(con: &mut DbPoolConnection) -> crate::Result<()> {
    reset_db_no_seed(con)?;
    seed(con)?;
    Ok(())
}
