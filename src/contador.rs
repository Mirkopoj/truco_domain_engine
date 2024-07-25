use std::fmt::Display;

use crate::{envidos::Envidos, equipos::Equipo};

pub struct Contador {
    puntos: [u8; 2],
    hasta: u8,
}

impl Contador {
    pub fn new(hasta: u8) -> Self {
        Self {
            puntos: [0; 2],
            hasta,
        }
    }

    pub fn hay_ganador(&self) -> bool {
        self.hasta <= self.puntos[0].max(self.puntos[1])
    }

    pub fn sumar(&mut self, ganador: Option<Equipo>, valor: u8) -> bool {
        if let Some(equipo) = ganador {
            match equipo {
                Equipo::Nosotros => {
                    self.puntos[0] += valor;
                }
                Equipo::Ellos => {
                    self.puntos[1] += valor;
                }
            };
        }
        self.hay_ganador()
    }

    pub fn sumar_tantos(&mut self, ganador: Equipo, tantos: Envidos) -> bool {
        let (ganador, perdedor) = match ganador {
            Equipo::Nosotros => (0, 1),
            Equipo::Ellos => (1, 0),
        };
        let valor = match tantos {
            Envidos::Value(t) => t,
            Envidos::Falta => {
                let rival = self.puntos[perdedor];
                ((rival / 15) + 1) * 15 - rival
            }
        };
        self.puntos[ganador] += valor;
        self.hay_ganador()
    }

    pub fn ganador(&self) -> Option<Equipo> {
        if self.hasta <= self.puntos[0] {
            return Some(Equipo::Nosotros);
        }
        if self.hasta <= self.puntos[1] {
            return Some(Equipo::Ellos);
        }
        None
    }
}

fn cuadradito(puntos: u8) -> (char, char, char, char, char) {
    let n_1 = if puntos > 0 { '▁' } else { ' ' };
    let n_2 = if puntos > 1 { '▕' } else { ' ' };
    let n_3 = if puntos > 2 { '▏' } else { ' ' };
    let n_4 = if puntos > 3 { '▔' } else { ' ' };
    let n_5 = if puntos > 4 { '╱' } else { ' ' };
    (n_1, n_2, n_3, n_4, n_5)
}

impl Display for Contador {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, " Nosotros │ Ellos")?;
            writeln!(f, "──────────┼──────────")?;

            let limit = self.hasta.div_ceil(5);

            for i in 0..limit {
                let (n_1, n_2, n_3, n_4, n_5) = cuadradito(self.puntos[0].saturating_sub(i * 5));
                let (e_1, e_2, e_3, e_4, e_5) = cuadradito(self.puntos[1].saturating_sub(i * 5));
                writeln!(f, "    {n_1}     │    {e_1}     ")?;
                writeln!(f, "   {n_2}{n_5}{n_3}    │   {e_2}{e_5}{e_3}    ")?;
                writeln!(f, "    {n_4}     │    {e_4}     ")?;
                if i == 3 {
                    writeln!(f, "──────────┼──────────")?;
                }
            }
            write!(f, "")
        } else {
            write!(f, "{} {}", self.puntos[0], self.puntos[1])
        }
    }
}
