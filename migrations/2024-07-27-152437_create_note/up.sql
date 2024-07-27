-- Your SQL goes here
create table note (
    id_note UUID primary key default gen_random_uuid(),
    etudiant text not null references etudiant(etu),
    matiere text not null references matiere(id_matiere),
    submission TIMESTAMP not null default now(),
    valeur decimal not null
);