use std::cell::Cell;

use estados::{inicial::Inicial, Envidos, TrucoState};

const MAX_PLAYERS: usize = 6;

pub struct TrucoStateMachine {
    internal_state: Cell<Option<Box<dyn TrucoState>>>,
}

impl TrucoStateMachine {
    /// # Errors
    /// # Panics
    pub fn irse_al_maso(&mut self) -> Result<(), &str> {
        match self.internal_state.take().unwrap().irse_al_maso() {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_quiero(&mut self, player: &str) -> Result<(), &str> {
        match self.internal_state.take().unwrap().cantar_quiero(player) {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_no_quiero(&mut self, player: &str) -> Result<(), &str> {
        match self.internal_state.take().unwrap().cantar_no_quiero(player) {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_envido(&mut self, player: &str) -> Result<(), &str> {
        match self.internal_state.take().unwrap().cantar_envido(player) {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_real_envido(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .unwrap()
            .cantar_real_envido(player)
        {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_falta_envido(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .unwrap()
            .cantar_falta_envido(player)
        {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_truco(&mut self, player: &str) -> Result<(), &str> {
        match self.internal_state.take().unwrap().cantar_truco(player) {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_re_truco(&mut self, player: &str) -> Result<(), &str> {
        match self.internal_state.take().unwrap().cantar_re_truco(player) {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn cantar_vale_cuatro(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .unwrap()
            .cantar_vale_cuatro(player)
        {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn tirar_carta(&mut self, player: &str) -> Result<(), &str> {
        match self.internal_state.take().unwrap().tirar_carta(player) {
            Ok(s) => {
                self.internal_state.set(Some(s));
                Ok(())
            }
            Err(s) => {
                self.internal_state.set(Some(s));
                Err("Invalid state transition")
            }
        }
    }

    /// # Errors
    /// # Panics
    pub fn tantos(&mut self) -> Result<Envidos, &str> {
        self.internal_state.get_mut().as_ref().unwrap().tantos()
    }

    /// # Errors
    /// # Panics
    pub fn valor_ronda(&mut self) -> Result<u8, &str> {
        match self
            .internal_state
            .get_mut()
            .as_ref()
            .unwrap()
            .valor_ronda()
        {
            Ok(v) => Ok(v as u8),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct TrucoStateMachineBuilder {
    players: Vec<String>,
}

impl TrucoStateMachineBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: String) {
        self.players.push(player);
    }

    /// # Errors
    pub fn build(self) -> Result<TrucoStateMachine, &'static str> {
        let player_count = self.players.len();
        if player_count <= MAX_PLAYERS && player_count % 2 == 0 {
            Ok(TrucoStateMachine {
                internal_state: Cell::new(Some(Box::new(Inicial::new(self.players)))),
            })
        } else {
            Err("Invalid player_count")
        }
    }
}

impl Default for TrucoStateMachineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

mod estados;
