use std::cell::Cell;
use std::marker::PhantomData;

use crate::equipos::Equipo;
use crate::{carta::Carta, envidos::Envidos};
use estados::{inicial::Inicial, TrucoState};

pub(super) struct TrucoStateMachine {
    internal_state: Cell<Option<Box<dyn TrucoState>>>,
}

impl TrucoStateMachine {
    const NON_EXISTING_STATE: &'static str =
        "Tried to use non existing state. This should never happend";

    /// # Errors
    /// # Panics
    pub fn irse_al_maso(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_quiero(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_no_quiero(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_envido(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_real_envido(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_falta_envido(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_truco(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_re_truco(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn cantar_vale_cuatro(&mut self, player: &str) -> Result<(), &'static str> {
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
    pub fn tirar_carta(&mut self, player: &str, card: Carta) -> Result<(), &'static str> {
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
    pub fn tantos(&mut self) -> Result<Envidos, &'static str> {
        self.internal_state
            .get_mut()
            .as_ref()
            .expect(Self::NON_EXISTING_STATE)
            .tantos()
    }

    /// # Errors
    /// # Panics
    pub fn valor_ronda(&mut self) -> Result<u8, &'static str> {
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
    pub fn winner(&mut self) -> Result<Option<Equipo>, &'static str> {
        self.internal_state
            .get_mut()
            .as_ref()
            .expect(Self::NON_EXISTING_STATE)
            .winner()
    }

    pub fn rebuild(players: Vec<String>, mano: usize) -> Self {
        Self {
            internal_state: Cell::new(Some(Box::new(Inicial::new(players, mano)))),
        }
    }
}

#[derive(Debug)]
pub(super) struct TrucoStateMachineBuilder<C: Cont> {
    players: Vec<String>,
    marker: std::marker::PhantomData<C>,
}

impl TrucoStateMachineBuilder<Cero> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            marker: PhantomData,
        }
    }
}

impl TrucoStateMachineBuilder<Cero> {
    pub fn add_player(mut self, player: String) -> TrucoStateMachineBuilder<Uno> {
        self.players.push(player);
        TrucoStateMachineBuilder::from(self)
    }
}

impl TrucoStateMachineBuilder<Uno> {
    pub fn add_player(mut self, player: String) -> TrucoStateMachineBuilder<Dos> {
        self.players.push(player);
        TrucoStateMachineBuilder::from(self)
    }
}

impl TrucoStateMachineBuilder<Dos> {
    pub fn add_player(mut self, player: String) -> TrucoStateMachineBuilder<Tres> {
        self.players.push(player);
        TrucoStateMachineBuilder::from(self)
    }
}

impl TrucoStateMachineBuilder<Tres> {
    pub fn add_player(mut self, player: String) -> TrucoStateMachineBuilder<Cuatro> {
        self.players.push(player);
        TrucoStateMachineBuilder::from(self)
    }
}

impl TrucoStateMachineBuilder<Cuatro> {
    pub fn add_player(mut self, player: String) -> TrucoStateMachineBuilder<Cinco> {
        self.players.push(player);
        TrucoStateMachineBuilder::from(self)
    }
}

impl TrucoStateMachineBuilder<Cinco> {
    pub fn add_player(mut self, player: String) -> TrucoStateMachineBuilder<Seis> {
        self.players.push(player);
        TrucoStateMachineBuilder::from(self)
    }
}

impl<C: Cont + Buildable> TrucoStateMachineBuilder<C> {
    /// # Errors
    pub fn build(self) -> TrucoStateMachine {
        TrucoStateMachine {
            internal_state: Cell::new(Some(Box::new(Inicial::new(self.players, 0)))),
        }
    }
}

impl<Co: Cont> TrucoStateMachineBuilder<Co> {
    fn from<Ci: Cont>(value: TrucoStateMachineBuilder<Ci>) -> TrucoStateMachineBuilder<Co> {
        TrucoStateMachineBuilder::<Co> {
            players: value.players,
            marker: PhantomData,
        }
    }
}

impl Default for TrucoStateMachineBuilder<Cero> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Cont {}
pub struct Cero;
impl Cont for Cero {}
pub struct Uno;
impl Cont for Uno {}
pub struct Dos;
impl Cont for Dos {}
pub struct Tres;
impl Cont for Tres {}
pub struct Cuatro;
impl Cont for Cuatro {}
pub struct Cinco;
impl Cont for Cinco {}
pub struct Seis;
impl Cont for Seis {}

pub trait Buildable {}
impl Buildable for Dos {}
impl Buildable for Cuatro {}
impl Buildable for Seis {}

mod estados;
