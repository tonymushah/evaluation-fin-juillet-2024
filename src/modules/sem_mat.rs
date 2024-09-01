use crate::models::table::Matiere;

use diesel::prelude::*;
use diesel_schemas::schema::matiere;

#[derive(Debug, Clone)]
pub enum SemestreMatiere {
    Unique(Matiere),
    Choix(Vec<Matiere>),
}

impl SemestreMatiere {
    pub fn credits(&self) -> i32 {
        match self {
            Self::Unique(m) => m.credits,
            Self::Choix(m) => m.first().map(|m| m.credits).unwrap_or_default(),
        }
    }
    pub fn type_(&self) -> Option<SemestreMatiereType> {
        match self {
            Self::Unique(m) => m.type_(),
            Self::Choix(m) => {
                let mut typs = m
                    .iter()
                    .flat_map(|mat| mat.type_())
                    .collect::<Vec<SemestreMatiereType>>();
                typs.dedup();
                if typs.len() != 1 {
                    None
                } else {
                    typs.first().copied()
                }
            }
        }
    }
    pub fn fold_matieres<I: Iterator<Item = Matiere>>(iter: I) -> Vec<SemestreMatiere> {
        let res = iter.fold(Vec::<Self>::new(), |mut acc, mat| {
            if mat.optionel.unwrap_or_default() {
                if let Some(Self::Choix(choix)) = acc.iter_mut().find(|mati| {
                    let Some(mati_type_) = mati.type_() else {
                        return false;
                    };
                    let Some(mat_type_) = mat.type_() else {
                        return false;
                    };
                    mat_type_ == mati_type_ && mati.credits() == mat.credits
                }) {
                    choix.push(mat);
                } else {
                    acc.push(Self::Choix(vec![mat]));
                }
            } else {
                acc.push(Self::Unique(mat));
            }
            acc
        });
        res
    }
}

#[derive(Debug, Clone)]
pub struct SemestreMatieres(pub Vec<SemestreMatiere>);

impl SemestreMatieres {
    pub fn from_semestres(con: &mut PgConnection, semestre_: String) -> QueryResult<Self> {
        use self::matiere::dsl::*;
        let matieres: Vec<Matiere> = matiere
            .filter(semestre.eq(semestre_))
            .select(Matiere::as_select())
            .load(con)?;
        let sems_mat = SemestreMatiere::fold_matieres(matieres.into_iter());
        Ok(Self(sems_mat))
    }
    pub fn is_matiere_there(&self, mat: &String) -> bool {
        self.0.iter().any(|sem_mat| match sem_mat {
            SemestreMatiere::Choix(mats) => mats.iter().any(|mat_| &mat_.id_matiere == mat),
            SemestreMatiere::Unique(mat_) => &mat_.id_matiere == mat,
        })
    }
    pub fn is_matiere_optionel(&self, mat: &String) -> Option<bool> {
        if self.is_matiere_there(mat) {
            None
        } else {
            Some(self.0.iter().any(|sem_mat| match sem_mat {
                SemestreMatiere::Choix(mats) => mats.iter().any(|mat_| &mat_.id_matiere == mat),
                SemestreMatiere::Unique(_) => false,
            }))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SemestreMatiereType {
    Info,
    Math,
    Org,
}

impl Matiere {
    pub fn type_(&self) -> Option<SemestreMatiereType> {
        let binding = self
            .id_matiere
            .chars()
            .map(|char| if char.is_ascii_digit() { ' ' } else { char })
            .collect::<String>();
        let ty = binding.split(" ").next()?;
        match ty {
            "INF" => Some(SemestreMatiereType::Info),
            "MTH" => Some(SemestreMatiereType::Math),
            "ORG" => Some(SemestreMatiereType::Org),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SemestreMatiere, SemestreMatiereType};
    use crate::models::table::Matiere;

    #[test]
    fn test_matiere_type_parse() {
        let mat = Matiere {
            id_matiere: "INF310".into(),
            credits: 4,
            semestre: "S6".into(),
            optionel: Some(false),
            nom: "Codage".into(),
        };
        assert_eq!(mat.type_(), Some(SemestreMatiereType::Info))
    }
    #[test]
    fn test_matiere_type_parse_not() {
        let mat = Matiere {
            id_matiere: "HEHE".into(),
            credits: 4,
            semestre: "S6".into(),
            optionel: Some(false),
            nom: "Codage".into(),
        };
        assert_eq!(mat.type_(), None)
    }

    use diesel::prelude::*;
    #[test]
    fn test_fold() -> anyhow::Result<()> {
        use diesel_schemas::schema::matiere::dsl::*;
        let pool = crate::etablish_connection();
        let mut con = pool.get()?;
        let matieres: Vec<Matiere> = matiere
            .filter(semestre.eq("S4"))
            .select(Matiere::as_select())
            .load(&mut con)?;
        let sems_mat = SemestreMatiere::fold_matieres(matieres.into_iter());
        assert_eq!(sems_mat.len(), 5);
        assert_eq!(sems_mat.iter().map(|mat| mat.credits()).sum::<i32>(), 30);
        Ok(())
    }
}
