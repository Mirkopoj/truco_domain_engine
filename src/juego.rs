use crate::{
    carta::Carta,
    envidos::Envidos,
    equipos::Equipo,
    jugador::Jugador,
    maquina_de_estados::{TrucoStateMachine, TrucoStateMachineBuilder},
    mazo::Mazo,
};

pub struct Truco {
    jugadores: Vec<Jugador>,
    estado: TrucoStateMachine,
    mazo: Mazo,
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
    pub fn tirar_carta(&mut self, player: &str, card: Carta) -> Result<(), &str> {
        self.estado.tirar_carta(player, card)?;
        self.reset_if_needed()
    }

    fn tantos(&mut self) -> Result<Envidos, &'static str> {
        self.estado.tantos()
    }

    fn valor_ronda(&mut self) -> Result<u8, &'static str> {
        self.estado.valor_ronda()
    }

    /// # Panics
    pub fn valid_commands(&mut self, player: &'static str) -> Vec<String> {
        self.estado.valid_commands(player)
    }

    fn winner(&mut self) -> Result<Option<Equipo>, &'static str> {
        self.estado.winner()
    }

    fn repartir(&mut self) {
        self.mazo.mezclar();
        self.mazo.repartir(&mut self.jugadores);
    }

    fn rebuild_state(&mut self) {
        let mut builder = TrucoStateMachineBuilder::new();
        for jugador in &self.jugadores {
            builder.add_player(jugador.nombre());
        }
        self.estado = builder.build().expect("player amount should be valid");
    }

    fn new_round(&mut self) -> Result<(Envidos, u8), &'static str> {
        let tantos = self.tantos()?;
        let ronda = self.valor_ronda()?;
        self.rebuild_state();
        self.repartir();
        Ok((tantos, ronda))
    }

    fn reset_if_needed(&mut self) -> Result<(), &str>{
        if let Ok(_winner) = self.winner() {
            self.new_round()?;
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct TrucoBuilder {
    jugadores: Vec<Jugador>,
    state_builder: TrucoStateMachineBuilder,
}

impl TrucoBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            jugadores: Vec::new(),
            state_builder: TrucoStateMachineBuilder::new(),
        }
    }

    pub fn add_player(&mut self, player: String) {
        self.jugadores.push(Jugador::new(player.clone()));
        self.state_builder.add_player(player);
    }

    /// # Errors
    pub fn build(self) -> Result<Truco, &'static str> {
        Ok(Truco {
            jugadores: self.jugadores,
            estado: self.state_builder.build()?,
            mazo: Mazo::new(),
        })
    }
}

impl Default for TrucoBuilder {
    fn default() -> Self {
        Self::new()
    }
}
