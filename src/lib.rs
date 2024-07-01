use estados::{inicial::Inicial, Envidos, TrucoState};

const MAX_PLAYERS: usize = 6;

pub struct TrucoStateMachine {
    internal_state: Box<dyn TrucoState>,
}

impl TrucoStateMachine {
    pub fn irse_al_maso(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.irse_al_maso()?;
        Ok(())
    }

    pub fn cantar_quiero(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_quiero(player)?;
        Ok(())
    }

    pub fn cantar_no_quiero(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_no_quiero(player)?;
        Ok(())
    }

    pub fn cantar_envido(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_envido(player)?;
        Ok(())
    }

    pub fn cantar_real_envido(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_real_envido(player)?;
        Ok(())
    }

    pub fn cantar_falta_envido(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_falta_envido(player)?;
        Ok(())
    }

    pub fn cantar_truco(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_truco(player)?;
        Ok(())
    }

    pub fn cantar_re_truco(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_re_truco(player)?;
        Ok(())
    }

    pub fn cantar_vale_cuatro(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_vale_cuatro(player)?;
        Ok(())
    }

    pub fn tirar_carta(&mut self, player: &str) -> Result<(), ()> {
        self.internal_state = self.internal_state.tirar_carta(player)?;
        Ok(())
    }

    pub fn tantos(&self) -> Result<Envidos, &str> {
        self.internal_state.tantos()
    }

    pub fn valor_ronda(&self) -> Result<u8, &str> {
        match self.internal_state.valor_ronda() {
            Ok(v) => Ok(v as u8),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
struct TrucoStateMachineBuilder {
    players: Vec<String>,
}

impl TrucoStateMachineBuilder {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: String) {
        self.players.push(player);
    }

    pub fn build(self) -> Result<TrucoStateMachine, &'static str> {
        let player_count = self.players.len();
        if player_count <= MAX_PLAYERS && player_count % 2 == 0 {
            Ok(TrucoStateMachine {
                internal_state: Box::new(Inicial::new(self.players)),
            })
        } else {
            Err("Invalid player_count")
        }
    }
}

mod estados;
