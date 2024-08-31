pub mod auth;
pub mod cfg_note;
pub mod database;
pub mod etudiants;
pub mod getters;
pub mod hello;
pub mod imports;
pub mod notes;

pub use auth::AuthService;
pub use database::DatabaseService;
pub use etudiants::EtudiantsService;
pub use getters::GettersService;
pub use hello::HelloServ;
pub use imports::ImportService;
pub use notes::NotesService;
