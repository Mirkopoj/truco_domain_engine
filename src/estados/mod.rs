use std::ops;

pub(super) trait TrucoState {
    fn irse_al_maso(self) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_quiero(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_no_quiero(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_envido(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_real_envido(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_falta_envido(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_truco(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_re_truco(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn cantar_vale_cuatro(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
    fn tirar_carta(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>>;
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

#[derive(Debug, Clone)]
struct Players {
    cont: usize,
    players: Vec<String>,
}

impl Players {
    fn new(players: Vec<String>) -> Self {
        Self { cont: 0, players }
    }

    fn is_turn(&self, player: &str) -> bool {
        player == self.players[self.cont]
    }

    /// Returns true if that was the last player in
    /// the round and the counter went back to 0
    fn next_player(&mut self) -> bool {
        self.cont += 1;
        self.cont %= self.players.len();
        self.cont == 0
    }

    fn is_team(&self, player: &str) -> bool {
        if let Some(index) = self.players.iter().position(|p| p == player) {
            index % 2 == self.cont % 2
        } else {
            false
        }
    }
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
