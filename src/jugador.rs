use crate::carta::Carta;

#[derive(Debug)]
pub struct Jugador {
    nombre: String,
    mano: [Option<Carta>; 3],
}

impl Jugador {
    pub fn new(nombre: String) -> Self {
        Self {
            nombre,
            mano: [None; 3],
        }
    }

    pub fn dar_mano(&mut self, mano: [Carta; 3]) {
        self.mano = mano.map(Some);
    }

    pub fn nombre(&self) -> String {
        self.nombre.clone()
    }
}
