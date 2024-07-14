use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Envidos {
    Value(u8),
    Falta,
}

impl ops::Add<u8> for Envidos {
    type Output = Envidos;
    fn add(self, rh: u8) -> Envidos {
        match self {
            Envidos::Value(e) => Envidos::Value(e + rh),
            Envidos::Falta => self,
        }
    }
}

