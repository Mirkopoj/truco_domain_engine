use itertools::Itertools;
use std::fmt::Display;

use crate::carta::Carta;

#[derive(Debug, Clone)]
pub struct Jugador {
    nombre: String,
    mano: [Option<Carta>; 3],
    tantos: u8,
}

impl Jugador {
    pub fn new(nombre: &str) -> Self {
        Self {
            nombre: nombre.trim().to_string(),
            mano: [None; 3],
            tantos: 0,
        }
    }

    pub fn dar_mano(&mut self, mano: [Carta; 3]) {
        self.tantos = mano
            .iter()
            .combinations(2)
            .map(|par| par.iter().map(|carta| carta.valor_tantos()).sum())
            .max()
            .expect("mano.iter() Should never yeild an empty iterator");
        self.mano = mano.map(Some);
    }

    pub fn nombre(&self) -> String {
        self.nombre.clone()
    }

    pub fn tantos(&self) -> u8 {
        self.tantos
    }

    pub fn carta(&mut self, carta: usize) -> Result<Carta, &'static str> {
        let msg = "Esa carta no existe";
        if carta < 3 && self.mano[carta].is_some() {
            Ok(self.mano[carta].unwrap())
        } else {
            Err(msg)
        }
    }

    pub fn use_carta(&mut self, carta: usize) {
        self.mano[carta].take();
    }
}

impl Display for Jugador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for carta in self.mano {
            match carta {
                Some(c) => write!(f, " {c}"),
                None => write!(f, "    "),
            }?;
        }
        write!(f, "")
    }
}
