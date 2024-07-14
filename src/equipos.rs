use std::ops::Not;

#[derive(Debug, Clone, Copy)]
pub enum Equipo {
    Nosotros,
    Ellos,
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
