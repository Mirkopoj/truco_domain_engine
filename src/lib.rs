use std::ops;

use inicial::Inicial;

trait TrucoState {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_envido(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_real_envido(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_falta_envido(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_truco(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_re_truco(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn cantar_vale_cuatro(&self) -> Result<Box<dyn TrucoState>, ()>;
    fn tirar_carta(&mut self) -> Result<Box<dyn TrucoState>, ()>;
    fn tantos(&self) -> Result<Envidos, &str>;
    fn valor_ronda(&self) -> Result<u8, &str>;
}

#[derive(Debug, Clone, Copy)]
pub enum CantidadDeJugadores {
    Dos = 2,
    Cuatro = 4,
    Seis = 6,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Envidos {
    Value(u8),
    Falta,
}

impl ops::Add<u8> for Envidos {
    type Output = Envidos;
    fn add(self, rh: u8) -> Envidos {
        match self {
            Envidos::Value(e) => Envidos::Value(e + rh),
            Envidos::Falta => self,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Trucos {
    Simple = 1,
    Truco = 2,
    ReTruco = 3,
    ValeCuatro = 4,
}

pub struct TrucoStateMachine {
    internal_state: Box<dyn TrucoState>,
}

impl TrucoStateMachine {
    #[must_use]
    pub fn new(de_a: CantidadDeJugadores) -> Self {
        Self {
            internal_state: Box::new(Inicial::new(de_a)),
        }
    }

    pub fn irse_al_maso(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.irse_al_maso()?;
        Ok(())
    }

    pub fn cantar_quiero(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_quiero()?;
        Ok(())
    }

    pub fn cantar_no_quiero(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_no_quiero()?;
        Ok(())
    }

    pub fn cantar_envido(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_envido()?;
        Ok(())
    }

    pub fn cantar_real_envido(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_real_envido()?;
        Ok(())
    }

    pub fn cantar_falta_envido(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_falta_envido()?;
        Ok(())
    }

    pub fn cantar_truco(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_truco()?;
        Ok(())
    }

    pub fn cantar_re_truco(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_re_truco()?;
        Ok(())
    }

    pub fn cantar_vale_cuatro(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_vale_cuatro()?;
        Ok(())
    }

    pub fn tirar_carta(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.tirar_carta()?;
        Ok(())
    }

    pub fn tantos(&self) -> Result<Envidos, &str> {
        self.internal_state.tantos()
    }

    pub fn valor_ronda(&self) -> Result<u8, &str> {
        self.internal_state.valor_ronda()
    }
}

mod envido;
mod envido_envido;
mod envido_va_primero;
mod falta_envido;
mod r#final;
mod inicial;
mod nada;
mod re_truco;
mod re_truco_querido;
mod real_envido;
mod truco;
mod truco_querido;
mod vale_cuatro;
mod vale_cuatro_querido;

#[cfg(test)]
mod tests;
