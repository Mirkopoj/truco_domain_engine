use std::cell::Cell;

use crate::carta::Carta;
use crate::equipos::Equipo;
use estados::{inicial::Inicial, Envidos, TrucoState};

const MAX_PLAYERS: usize = 6;

pub struct TrucoStateMachine {
    internal_state: Cell<Option<Box<dyn TrucoState>>>,
}

impl TrucoStateMachine {
    const NON_EXISTING_STATE: &'static str =
        "Tried to use non existing state. This should never happend";

    /// # Errors
    /// # Panics
    pub fn irse_al_maso(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
            .irse_al_maso(player)
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
    pub fn cantar_quiero(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
            .cantar_quiero(player)
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
    pub fn cantar_no_quiero(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
            .cantar_no_quiero(player)
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
    pub fn cantar_envido(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
            .cantar_envido(player)
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
    pub fn cantar_real_envido(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
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
            .expect(Self::NON_EXISTING_STATE)
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
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
            .cantar_truco(player)
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
    pub fn cantar_re_truco(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
            .cantar_re_truco(player)
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
    pub fn cantar_vale_cuatro(&mut self, player: &str) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
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
    pub fn tirar_carta(&mut self, player: &str, card: Carta) -> Result<(), &str> {
        match self
            .internal_state
            .take()
            .expect(Self::NON_EXISTING_STATE)
            .tirar_carta(player, card)
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
    pub fn tantos(&mut self) -> Result<Envidos, &str> {
        self.internal_state
            .get_mut()
            .as_ref()
            .expect(Self::NON_EXISTING_STATE)
            .tantos()
    }

    /// # Errors
    /// # Panics
    pub fn valor_ronda(&mut self) -> Result<u8, &str> {
        match self
            .internal_state
            .get_mut()
            .as_ref()
            .expect(Self::NON_EXISTING_STATE)
            .valor_ronda()
        {
            Ok(v) => Ok(v as u8),
            Err(e) => Err(e),
        }
    }

    /// # Panics
    pub fn valid_commands(&mut self, player: &str) -> Vec<String> {
        self.internal_state
            .get_mut()
            .as_ref()
            .expect(Self::NON_EXISTING_STATE)
            .valid_commands(player)
    }

    /// # Errors
    /// # Panics
    pub fn winner(&mut self) -> Result<Option<Equipo>, &str> {
        self.internal_state
            .get_mut()
            .as_ref()
            .expect(Self::NON_EXISTING_STATE)
            .winner()
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
