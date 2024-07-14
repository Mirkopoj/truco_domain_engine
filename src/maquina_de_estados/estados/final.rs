use crate::{carta::Carta, equipos::Equipo};

use super::{Envidos, TrucoState, Trucos};

#[derive(Debug, Clone, Copy)]
pub struct Final {
    tantos: Envidos,
    valor_ronda: Trucos,
    winner: Option<Equipo>,
}

impl Final {
    pub fn new(tantos: Envidos, valor_ronda: Trucos, winner: Option<Equipo>) -> Self {
        Self {
            tantos,
            valor_ronda,
            winner,
        }
    }
}

impl TrucoState for Final {
    fn irse_al_maso(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_quiero(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_no_quiero(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_envido(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_real_envido(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_falta_envido(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_truco(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_re_truco(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_vale_cuatro(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn tirar_carta(
        self: Box<Self>,
        _: &str,
        _: Carta,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn tantos(&self) -> Result<Envidos, &'static str> {
        Ok(self.tantos)
    }

    fn valor_ronda(&self) -> Result<Trucos, &'static str> {
        Ok(self.valor_ronda)
    }

    fn valid_commands(&self, _: &str) -> Vec<String> {
        Vec::new()
    }

    fn winner(&self) -> Result<Option<Equipo>, &'static str> {
        Ok(self.winner)
    }
}
