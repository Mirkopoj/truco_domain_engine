use enum_iterator::Sequence;
use std::fmt::Display;

#[derive(Sequence, Debug, Clone, Copy, PartialEq)]
pub enum Palo {
    Espada,
    Basto,
    Oro,
    Copa,
}

#[derive(Sequence, Debug, Clone, Copy, PartialEq)]
pub enum Numero {
    Ancho,
    Dos,
    Tres,
    Cuatro,
    Cinco,
    Seis,
    Siete,
    Sota,
    Caballo,
    Rey,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Carta {
    palo: Palo,
    numero: Numero,
    valor_tantos: u8,
    valor_juego: u8,
}

fn valor_juego(numero: Numero, palo: Palo) -> u8 {
    match (numero, palo) {
        (Numero::Ancho, Palo::Espada) => 14,
        (Numero::Ancho, Palo::Basto) => 13,
        (Numero::Siete, Palo::Espada) => 12,
        (Numero::Siete, Palo::Oro) => 11,
        (Numero::Tres, _) => 10,
        (Numero::Dos, _) => 9,
        (Numero::Ancho, Palo::Copa | Palo::Oro) => 8,
        (Numero::Rey, _) => 7,
        (Numero::Caballo, _) => 6,
        (Numero::Sota, _) => 5,
        (Numero::Siete, Palo::Copa | Palo::Basto) => 4,
        (Numero::Seis, _) => 3,
        (Numero::Cinco, _) => 2,
        (Numero::Cuatro, _) => 1,
    }
}

fn valor_tantos(numero: Numero) -> u8 {
    match numero {
        Numero::Ancho => 1,
        Numero::Dos => 2,
        Numero::Tres => 3,
        Numero::Cuatro => 4,
        Numero::Cinco => 5,
        Numero::Seis => 6,
        Numero::Siete => 7,
        _ => 0,
    }
}

impl Carta {
    pub fn new(numero: Numero, palo: Palo) -> Carta {
        let tantos = valor_tantos(numero);
        let valor = valor_juego(numero, palo);
        Carta {
            palo,
            numero,
            valor_juego: valor,
            valor_tantos: tantos,
        }
    }

    pub fn valor_tantos(self) -> u8 {
        self.valor_tantos
    }

    pub fn valor_juego(self) -> u8 {
        self.valor_juego
    }
}

impl Display for Carta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.numero {
            Numero::Ancho => write!(f, " 1"),
            Numero::Dos => write!(f, " 2"),
            Numero::Tres => write!(f, " 3"),
            Numero::Cuatro => write!(f, " 4"),
            Numero::Cinco => write!(f, " 5"),
            Numero::Seis => write!(f, " 6"),
            Numero::Siete => write!(f, " 7"),
            Numero::Sota => write!(f, "10"),
            Numero::Caballo => write!(f, "11"),
            Numero::Rey => write!(f, "12"),
        }?;
        match self.palo {
            Palo::Espada => write!(f, "E"),
            Palo::Basto => write!(f, "B"),
            Palo::Copa => write!(f, "C"),
            Palo::Oro => write!(f, "O"),
        }
    }
}
