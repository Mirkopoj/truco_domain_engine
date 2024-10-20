use std::marker::PhantomData;

use crate::{
    carta::Carta,
    contador::Contador,
    envidos::Envidos,
    equipos::Equipo,
    jugador::Jugador,
    maquina_de_estados::{
        Buildable, Cero, Cinco, Cont, Cuatro, Dos, Seis, Tres, TrucoStateMachine,
        TrucoStateMachineBuilder, Uno,
    },
    mazo::Mazo,
};

pub struct Truco {
    jugadores: Vec<Jugador>,
    estado: TrucoStateMachine,
    mazo: Mazo,
    contador: Contador,
    mano: usize,
    terminado: bool,
    cartas: Vec<[Option<Carta>; 3]>,
}

impl Truco {
    /// # Errors
    /// # Panics
    pub fn irse_al_maso(&mut self, player: &str) -> Result<(), &str> {
        self.estado.irse_al_maso(player)?;
        self.reset_if_needed()
    }

    /// # Errors
    /// # Panics
    pub fn cantar_quiero(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_quiero(player)
    }

    /// # Errors
    /// # Panics
    pub fn cantar_no_quiero(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_no_quiero(player)?;
        self.reset_if_needed()
    }

    /// # Errors
    /// # Panics
    pub fn cantar_envido(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_envido(player)
    }

    /// # Errors
    /// # Panics
    pub fn cantar_real_envido(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_real_envido(player)
    }

    /// # Errors
    /// # Panics
    pub fn cantar_falta_envido(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_falta_envido(player)
    }

    /// # Errors
    /// # Panics
    pub fn cantar_truco(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_truco(player)
    }

    /// # Errors
    /// # Panics
    pub fn cantar_re_truco(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_re_truco(player)
    }

    /// # Errors
    /// # Panics
    pub fn cantar_vale_cuatro(&mut self, player: &str) -> Result<(), &str> {
        self.estado.cantar_vale_cuatro(player)
    }

    /// # Errors
    /// # Panics
    pub fn tirar_carta(&mut self, player: &str, card: usize) -> Result<(), &str> {
        let jugador = self
            .jugadores
            .iter_mut()
            .find(|j| j.nombre() == player)
            .ok_or("Jugador inexistente")?;
        let carta = jugador.carta(card)?;
        self.estado.tirar_carta(player, carta)?;
        jugador.use_carta(card);
        let index = self
            .jugadores
            .iter()
            .position(|j| j.nombre() == player)
            .unwrap();
        for i in 0..3 {
            if self.cartas[index][i].is_none() {
                self.cartas[index][i] = Some(carta);
                break;
            }
        }
        self.reset_if_needed()
    }

    fn tantos(&mut self) -> Result<Envidos, &'static str> {
        self.estado.tantos()
    }

    fn valor_ronda(&mut self) -> Result<u8, &'static str> {
        self.estado.valor_ronda()
    }

    /// # Panics
    pub fn valid_commands(&mut self, player: &str) -> Vec<String> {
        self.estado.valid_commands(player)
    }

    fn winner(&mut self) -> Result<Option<Equipo>, &'static str> {
        self.estado.winner()
    }

    pub(crate) fn repartir(&mut self) {
        self.mazo.mezclar();
        self.mazo.repartir(&mut self.jugadores);
    }

    fn rebuild_state(&mut self) {
        self.mano = (self.mano + 1) % self.jugadores.len();
        self.estado = TrucoStateMachine::rebuild(
            self.jugadores.iter().map(Jugador::nombre).collect(),
            self.mano,
        );
    }

    fn new_round(&mut self) {
        self.rebuild_state();
        self.repartir();
        let mut cartas = Vec::new();
        for _ in &self.jugadores {
            cartas.push([None; 3]);
        }
        self.cartas = cartas;
    }

    fn reset_if_needed(&mut self) -> Result<(), &str> {
        if let Ok(winner) = self.winner() {
            let tantos_para = Equipo::from(
                self.jugadores
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by(|(_, x), (_, y)| x.tantos().cmp(&y.tantos()))
                    .expect("Players Shoudn't be empty")
                    .0,
            );
            let tantos = self.tantos()?;
            if self.contador.sumar_tantos(tantos_para, tantos) {
                self.terminado = true;
                return Ok(());
            }
            let ronda = self.valor_ronda()?;
            if self.contador.sumar(winner, ronda) {
                self.terminado = true;
                return Ok(());
            };
            self.new_round();
        };
        Ok(())
    }

    pub fn terminado(&self) -> bool {
        self.terminado
    }

    pub fn ganador(&self) -> Option<Equipo> {
        self.contador.ganador()
    }

    pub fn cartas(&self) -> String {
        let mut ret = String::new();
        for i in (0..3).rev() {
            for cartas in &self.cartas {
                ret += &if let Some(c) = cartas[i] {
                    c.to_string() + " "
                } else {
                    "    ".to_string()
                };
            }
            ret += "\n";
        }
        ret
    }

    pub fn print_state(&mut self) -> String {
        format!("{:#}", self.contador) + "\n" + &self.cartas() + "\n" + &self.estado.turn()
    }

    /// # Errors
    pub fn print_player(&self, player: &str) -> Result<String, &str> {
        Ok(self
            .jugadores
            .iter()
            .find(|j| j.nombre() == player)
            .ok_or("Jugador inexistente")?
            .to_string())
    }
}

#[derive(Debug, Clone)]
pub struct TrucoBuilder<H: Hasta, C: Cont> {
    jugadores: Vec<Jugador>,
    state_builder: TrucoStateMachineBuilder<C>,
    hasta: Option<u8>,
    marker: std::marker::PhantomData<H>,
}

impl TrucoBuilder<Sin, Cero> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            jugadores: Vec::new(),
            state_builder: TrucoStateMachineBuilder::new(),
            hasta: None,
            marker: PhantomData,
        }
    }
}

impl<H: Hasta> TrucoBuilder<H, Cero> {
    #[must_use]
    pub fn add_player(mut self, player: &str) -> TrucoBuilder<H, Uno> {
        self.jugadores.push(Jugador::new(player));
        TrucoBuilder {
            jugadores: self.jugadores,
            state_builder: self.state_builder.add_player(player),
            hasta: self.hasta,
            marker: PhantomData,
        }
    }
}

impl<H: Hasta> TrucoBuilder<H, Uno> {
    #[must_use]
    pub fn add_player(mut self, player: &str) -> TrucoBuilder<H, Dos> {
        self.jugadores.push(Jugador::new(player));
        TrucoBuilder {
            jugadores: self.jugadores,
            state_builder: self.state_builder.add_player(player),
            hasta: self.hasta,
            marker: PhantomData,
        }
    }
}

impl<H: Hasta> TrucoBuilder<H, Dos> {
    #[must_use]
    pub fn add_player(mut self, player: &str) -> TrucoBuilder<H, Tres> {
        self.jugadores.push(Jugador::new(player));
        TrucoBuilder {
            jugadores: self.jugadores,
            state_builder: self.state_builder.add_player(player),
            hasta: self.hasta,
            marker: PhantomData,
        }
    }
}

impl<H: Hasta> TrucoBuilder<H, Tres> {
    #[must_use]
    pub fn add_player(mut self, player: &str) -> TrucoBuilder<H, Cuatro> {
        self.jugadores.push(Jugador::new(player));
        TrucoBuilder {
            jugadores: self.jugadores,
            state_builder: self.state_builder.add_player(player),
            hasta: self.hasta,
            marker: PhantomData,
        }
    }
}

impl<H: Hasta> TrucoBuilder<H, Cuatro> {
    #[must_use]
    pub fn add_player(mut self, player: &str) -> TrucoBuilder<H, Cinco> {
        self.jugadores.push(Jugador::new(player));
        TrucoBuilder {
            jugadores: self.jugadores,
            state_builder: self.state_builder.add_player(player),
            hasta: self.hasta,
            marker: PhantomData,
        }
    }
}

impl<H: Hasta> TrucoBuilder<H, Cinco> {
    #[must_use]
    pub fn add_player(mut self, player: &str) -> TrucoBuilder<H, Seis> {
        self.jugadores.push(Jugador::new(player));
        TrucoBuilder {
            jugadores: self.jugadores,
            state_builder: self.state_builder.add_player(player),
            hasta: self.hasta,
            marker: PhantomData,
        }
    }
}

impl<C: Cont> TrucoBuilder<Sin, C> {
    #[must_use]
    pub fn hasta(mut self, hasta: u8) -> TrucoBuilder<Con, C> {
        self.hasta = Some(hasta);
        TrucoBuilder::from(self)
    }
}

impl<H: Hasta + Buildable, C: Cont + Buildable> TrucoBuilder<H, C> {
    #[must_use]
    pub fn build(self) -> Truco {
        let mut cartas = Vec::new();
        for _ in &self.jugadores {
            cartas.push([None; 3]);
        }
        let mut ret = Truco {
            jugadores: self.jugadores,
            estado: self.state_builder.build(),
            mazo: Mazo::new(),
            contador: Contador::new(self.hasta.unwrap_or(30)),
            mano: 0,
            terminado: false,
            cartas,
        };
        ret.repartir();
        ret
    }
}

impl Default for TrucoBuilder<Sin, Cero> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Ho: Hasta, C: Cont> TrucoBuilder<Ho, C> {
    fn from<Hi: Hasta>(value: TrucoBuilder<Hi, C>) -> TrucoBuilder<Ho, C> {
        TrucoBuilder::<Ho, C> {
            jugadores: value.jugadores,
            state_builder: value.state_builder,
            hasta: value.hasta,
            marker: PhantomData,
        }
    }
}

pub trait Hasta {}
#[derive(Clone)]
pub struct Sin;
impl Hasta for Sin {}
#[derive(Clone)]
pub struct Con;
impl Hasta for Con {}

impl Buildable for Con {}
