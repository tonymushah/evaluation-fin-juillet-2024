// @generated automatically by Diesel CLI.

diesel::table! {
    configuration_note (code) {
        code -> Text,
        config -> Text,
        valeur -> Numeric,
    }
}

diesel::table! {
    etudiant (etu) {
        etu -> Text,
        nom -> Text,
        prenom -> Text,
        date_naissance -> Date,
        promotion -> Text,
        genre -> Int4,
    }
}

diesel::table! {
    matiere (id_matiere) {
        id_matiere -> Text,
        credits -> Int4,
        semestre -> Text,
        optionel -> Nullable<Bool>,
        nom -> Text,
    }
}

diesel::table! {
    note (id_note) {
        id_note -> Uuid,
        etudiant -> Text,
        matiere -> Text,
        submission -> Timestamp,
        valeur -> Numeric,
    }
}

diesel::table! {
    promotion (id_promotion) {
        id_promotion -> Text,
        nom -> Nullable<Text>,
    }
}

diesel::table! {
    semestre (id_sem) {
        id_sem -> Text,
    }
}

diesel::joinable!(etudiant -> promotion (promotion));
diesel::joinable!(matiere -> semestre (semestre));
diesel::joinable!(note -> etudiant (etudiant));
diesel::joinable!(note -> matiere (matiere));

diesel::allow_tables_to_appear_in_same_query!(
    configuration_note,
    etudiant,
    matiere,
    note,
    promotion,
    semestre,
);
