diesel::table! {
    v_etudiant_note(etu, id_note) {
        etu -> Text,
        etu_nom -> Text,
        etu_prenom -> Text,
        etu_dtn -> Date,
        id_note -> Uuid,
        matiere -> Text,
        submission -> Timestamp,
        valeur -> Numeric
    }
}

diesel::table! {
    v_matiere_note(id_note) {
        id_note -> Uuid,
        etudiant -> Text,
        matiere -> Text,
        submission -> Timestamp,
        valeur -> Numeric,
        nom -> Text,
        semestre -> Text,
        credits -> Integer,
        optionel -> Bool
    }
}
