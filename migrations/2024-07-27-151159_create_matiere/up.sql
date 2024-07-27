-- Your SQL goes here
create table matiere (
    id_matiere text primary key,
    credits int not null,
    semestre int not null references semestre(id_sem),
    optionel boolean default false,
    nom text not null
);