use itertools::Itertools;

use crate::carta::Carta;

#[derive(Debug)]
pub struct Jugador {
    nombre: String,
    mano: [Option<Carta>; 3],
    tantos: u8,
}

impl Jugador {
    pub fn new(nombre: String) -> Self {
        Self {
            nombre,
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
        if carta < 3 {
            self.mano[carta].take().ok_or(msg)
        } else {
            Err(msg)
        }
    }
}
