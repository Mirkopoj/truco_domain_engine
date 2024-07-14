use std::ops;

use crate::{carta::Carta, equipos::Equipo};

pub(super) trait TrucoState {
    fn irse_al_maso(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_no_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_envido(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_real_envido(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_falta_envido(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_truco(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_re_truco(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_vale_cuatro(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn tirar_carta(
        self: Box<Self>,
        player: &str,
        card: Carta,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn tantos(&self) -> Result<Envidos, &str>;
    fn valor_ronda(&self) -> Result<Trucos, &str>;
    fn valid_commands(&self, player: &str) -> Vec<String>;
    fn winner(&self) -> Result<Option<Equipo>, &str>;
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
pub(super) enum Trucos {
    Simple = 1,
    Truco = 2,
    ReTruco = 3,
    ValeCuatro = 4,
}

mod envido;
mod envido_envido;
mod envido_va_primero;
mod falta_envido;
mod r#final;
pub(super) mod inicial;
mod nada;
mod player_state;
mod re_truco;
mod re_truco_querido;
mod real_envido;
mod truco;
mod truco_querido;
mod vale_cuatro;
mod vale_cuatro_querido;

#[cfg(test)]
mod tests;
