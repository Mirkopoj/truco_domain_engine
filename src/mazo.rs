use enum_iterator::all;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{
    carta::{Carta, Numero, Palo},
    jugador::Jugador,
};

#[derive(Debug)]
pub struct Mazo {
    cartas: Vec<Carta>,
}

impl Mazo {
    pub fn new() -> Mazo {
        Mazo {
            cartas: all::<Palo>()
                .flat_map(|pal| all::<Numero>().map(move |num| Carta::new(num, pal)))
                .collect(),
        }
    }

    pub fn mezclar(&mut self) {
        let mut rng = thread_rng();
        self.cartas.shuffle(&mut rng);
    }

    pub fn repartir(&self, jugadores: &mut [Jugador]) {
        let numero_de_jugadores = jugadores.len();
        for (numero_de_jugador, jugador) in jugadores.iter_mut().enumerate() {
            jugador.dar_mano([
                self.cartas[numero_de_jugador],
                self.cartas[numero_de_jugador + numero_de_jugadores],
                self.cartas[numero_de_jugador + numero_de_jugadores * 2],
            ]);
        }
    }
}
