use std::ops;

pub(super) trait TrucoState {
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
    fn valor_ronda(&self) -> Result<Trucos, &str>;
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
mod re_truco;
mod re_truco_querido;
mod real_envido;
mod truco;
mod truco_querido;
mod vale_cuatro;
mod vale_cuatro_querido;

#[cfg(test)]
mod tests;
