use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TypeCalculNote {
    #[default]
    Max,
    Moyenne,
}

#[derive(Debug)]
pub struct TypeCalculNoteParseError(pub(crate) ());

impl Display for TypeCalculNoteParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Failed to parse something into an TypeCalculNote enum")
    }
}

impl Error for TypeCalculNoteParseError {}

impl TryFrom<u8> for TypeCalculNote {
    type Error = TypeCalculNoteParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Max),
            2 => Ok(Self::Moyenne),
            _ => Err(TypeCalculNoteParseError(())),
        }
    }
}

impl TypeCalculNote {
    pub fn parse(value: u8) -> Result<Self, TypeCalculNoteParseError> {
        Self::try_from(value)
    }
}
