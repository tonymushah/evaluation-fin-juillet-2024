// @generated automatically by Diesel CLI.

diesel::table! {
    etudiant (etu) {
        etu -> Text,
        nom -> Text,
        prenom -> Text,
        date_naissance -> Date,
        promotion -> Text,
    }
}

diesel::table! {
    matiere (id_matiere) {
        id_matiere -> Text,
        credits -> Int4,
        semestre -> Int4,
        optionel -> Nullable<Bool>,
        nom -> Text,
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
        id_sem -> Int4,
    }
}

diesel::joinable!(etudiant -> promotion (promotion));
diesel::joinable!(matiere -> semestre (semestre));

diesel::allow_tables_to_appear_in_same_query!(
    etudiant,
    matiere,
    promotion,
    semestre,
);
