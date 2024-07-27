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
    promotion (id_promotion) {
        id_promotion -> Text,
        nom -> Nullable<Text>,
    }
}

diesel::joinable!(etudiant -> promotion (promotion));

diesel::allow_tables_to_appear_in_same_query!(
    etudiant,
    promotion,
);
