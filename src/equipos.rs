use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Equipo {
    Nosotros,
    Ellos,
}

impl Equipo {
    pub fn from(index: usize) -> Equipo {
        if index % 2 == 0 {
            Equipo::Nosotros
        } else {
            Equipo::Ellos
        }
    }
}

impl Not for Equipo {
    type Output = Equipo;

    fn not(self) -> Self::Output {
        match self {
            Equipo::Nosotros => Equipo::Ellos,
            Equipo::Ellos => Equipo::Nosotros,
        }
    }
}
