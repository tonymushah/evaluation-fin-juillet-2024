use diesel::{insert_into, prelude::*, PgConnection, QueryResult};

use crate::models::table::{Matiere, Semestre};

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

pub fn seed_matiere(con: &mut PgConnection) -> QueryResult<()> {
    use diesel_schemas::schema::matiere::dsl::*;
    let s1 = vec![
        Matiere {
            id_matiere: "INF101".into(),
            credits: 7,
            semestre: "S1".into(),
            optionel: Some(false),
            nom: "Programmation procédurale".into(),
        },
        Matiere {
            id_matiere: "INF104".into(),
            credits: 5,
            semestre: "S1".into(),
            optionel: Some(false),
            nom: "HTML et Introduction au Web".into(),
        },
        Matiere {
            id_matiere: "INF107".into(),
            credits: 4,
            semestre: "S1".into(),
            optionel: Some(false),
            nom: "Informatique de Base".into(),
        },
        Matiere {
            id_matiere: "MTH101".into(),
            credits: 4,
            semestre: "S1".into(),
            optionel: Some(false),
            nom: "Arithmétique et nombres".into(),
        },
        Matiere {
            id_matiere: "MTH102".into(),
            credits: 6,
            semestre: "S1".into(),
            optionel: Some(false),
            nom: "Analyse mathématique".into(),
        },
        Matiere {
            id_matiere: "ORG101".into(),
            credits: 6,
            semestre: "S1".into(),
            optionel: Some(false),
            nom: "Analyse mathématique".into(),
        },
    ];

    let s2 = vec![
        Matiere {
            id_matiere: "INF102".into(),
            credits: 5,
            semestre: "S2".into(),
            optionel: Some(false),
            nom: "Bases de données relationnelles".into(),
        },
        Matiere {
            id_matiere: "INF103".into(),
            credits: 5,
            semestre: "S2".into(),
            optionel: Some(false),
            nom: "Bases de l’administration système".into(),
        },
        Matiere {
            id_matiere: "INF105".into(),
            credits: 4,
            semestre: "S2".into(),
            optionel: Some(false),
            nom: "Maintenance matériel et logiciel".into(),
        },
        Matiere {
            id_matiere: "INF106".into(),
            credits: 6,
            semestre: "S2".into(),
            optionel: Some(false),
            nom: "Compléments de programmation".into(),
        },
        Matiere {
            id_matiere: "MTH103".into(),
            credits: 6,
            semestre: "S2".into(),
            optionel: Some(false),
            nom: "Calcul Vectoriel et Matriciel".into(),
        },
        Matiere {
            id_matiere: "MTH105".into(),
            credits: 4,
            semestre: "S2".into(),
            optionel: Some(false),
            nom: "Probabilité et Statistique".into(),
        },
    ];

    let s3 = vec![
        Matiere {
            id_matiere: "INF201".into(),
            credits: 6,
            semestre: "S3".into(),
            optionel: Some(false),
            nom: "Programmation orientée objet".into(),
        },
        Matiere {
            id_matiere: "INF202".into(),
            credits: 6,
            semestre: "S3".into(),
            optionel: Some(false),
            nom: "Bases de données objets".into(),
        },
        Matiere {
            id_matiere: "INF203".into(),
            credits: 4,
            semestre: "S3".into(),
            optionel: Some(false),
            nom: "Programmation système".into(),
        },
        Matiere {
            id_matiere: "INF208".into(),
            credits: 6,
            semestre: "S3".into(),
            optionel: Some(false),
            nom: "Réseaux informatiques".into(),
        },
        Matiere {
            id_matiere: "MTH201".into(),
            credits: 4,
            semestre: "S3".into(),
            optionel: Some(false),
            nom: "Méthodes numériques".into(),
        },
        Matiere {
            id_matiere: "ORG201".into(),
            credits: 4,
            semestre: "S3".into(),
            optionel: Some(false),
            nom: "Bases de gestion".into(),
        },
    ];

    let s4 = vec![
        Matiere {
            id_matiere: "INF204".into(),
            credits: 6,
            semestre: "S4".into(),
            optionel: Some(true),
            nom: "Système d’information géographique".into(),
        },
        Matiere {
            id_matiere: "INF205".into(),
            credits: 6,
            semestre: "S4".into(),
            optionel: Some(true),
            nom: "Système d’information".into(),
        },
        Matiere {
            id_matiere: "INF206".into(),
            credits: 6,
            semestre: "S4".into(),
            optionel: Some(true),
            nom: "Interface Homme/Machine".into(),
        },
        Matiere {
            id_matiere: "INF207".into(),
            credits: 6,
            semestre: "S4".into(),
            optionel: Some(false),
            nom: "Mini-projet de développement".into(),
        },
        Matiere {
            id_matiere: "INF210".into(),
            credits: 10,
            semestre: "S4".into(),
            optionel: Some(false),
            nom: "Mini-projet de développement".into(),
        },
        Matiere {
            id_matiere: "MTH204".into(),
            credits: 4,
            semestre: "S4".into(),
            optionel: Some(true),
            nom: "Système d’information géographique".into(),
        },
        Matiere {
            id_matiere: "MTH205".into(),
            credits: 4,
            semestre: "S4".into(),
            optionel: Some(true),
            nom: "Equations différentielles".into(),
        },
        Matiere {
            id_matiere: "MTH206".into(),
            credits: 4,
            semestre: "S4".into(),
            optionel: Some(true),
            nom: "Optimisation".into(),
        },
        Matiere {
            id_matiere: "MTH203".into(),
            credits: 4,
            semestre: "S4".into(),
            optionel: Some(false),
            nom: "MAO".into(),
        },
    ];

    let s5 = vec![
        Matiere {
            id_matiere: "INF301".into(),
            credits: 6,
            semestre: "S5".into(),
            optionel: Some(false),
            nom: "Architecture logicielle".into(),
        },
        Matiere {
            id_matiere: "INF304".into(),
            credits: 6,
            semestre: "S5".into(),
            optionel: Some(false),
            nom: "Développement pour mobiles".into(),
        },
        Matiere {
            id_matiere: "INF307".into(),
            credits: 6,
            semestre: "S5".into(),
            optionel: Some(false),
            nom: "Conception en modèle orienté objet".into(),
        },
        Matiere {
            id_matiere: "ORG301".into(),
            credits: 5,
            semestre: "S5".into(),
            optionel: Some(false),
            nom: "Gestion d’entreprise".into(),
        },
        Matiere {
            id_matiere: "ORG302".into(),
            credits: 4,
            semestre: "S5".into(),
            optionel: Some(false),
            nom: "Gestion de projets".into(),
        },
        Matiere {
            id_matiere: "ORG303".into(),
            credits: 3,
            semestre: "S5".into(),
            optionel: Some(false),
            nom: "Anglais pour les affaires".into(),
        },
    ];

    let s6 = vec![
        Matiere {
            id_matiere: "INF310".into(),
            credits: 4,
            semestre: "S6".into(),
            optionel: Some(false),
            nom: "Codage".into(),
        },
        Matiere {
            id_matiere: "INF313".into(),
            credits: 6,
            semestre: "S6".into(),
            optionel: Some(false),
            nom: "Programmation avancée, frameworks".into(),
        },
        Matiere {
            id_matiere: "INF302".into(),
            credits: 6,
            semestre: "S6".into(),
            optionel: Some(true),
            nom: "Technologies d’accès aux réseaux".into(),
        },
        Matiere {
            id_matiere: "INF303".into(),
            credits: 6,
            semestre: "S5".into(),
            optionel: Some(true),
            nom: "Multimédia".into(),
        },
        Matiere {
            id_matiere: "INF316".into(),
            credits: 10,
            semestre: "S6".into(),
            optionel: Some(false),
            nom: "Projet de développement".into(),
        },
        Matiere {
            id_matiere: "ORG303".into(),
            credits: 4,
            semestre: "S6".into(),
            optionel: Some(false),
            nom: "Communication d’entreprise".into(),
        },
    ];

    insert_into(matiere).values(s1).execute(con)?;
    insert_into(matiere).values(s2).execute(con)?;
    insert_into(matiere).values(s3).execute(con)?;
    insert_into(matiere).values(s4).execute(con)?;
    insert_into(matiere).values(s5).execute(con)?;
    insert_into(matiere).values(s6).execute(con)?;
    Ok(())
}

pub fn seed(con: &mut PgConnection) -> QueryResult<()> {
    seed_semestre(con)?;
    seed_semestre(con)?;
    Ok(())
}
