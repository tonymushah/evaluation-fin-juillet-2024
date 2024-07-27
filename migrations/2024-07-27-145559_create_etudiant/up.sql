-- Your SQL goes here
create table etudiant (
    etu text primary key,
    nom text not null,
    prenom text not null,
    date_naissance Date not null,
    promotion text not null references promotion(id_promotion)
);