use crate::carta::Carta;

use super::{
    player_state::PlayersState, r#final::Final, vale_cuatro::ValeCuatro, Envidos, TrucoState,
    Trucos,
};

#[derive(Debug, Clone)]
pub struct ReTrucoQuerido {
    tantos: Envidos,
    players: PlayersState,
}

impl ReTrucoQuerido {
    pub fn new(tantos: Envidos, players: PlayersState) -> Self {
        Self { tantos, players }
    }
}

impl TrucoState for ReTrucoQuerido {
    fn irse_al_maso(self: Box<Self>) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Ok(Box::new(Final::new(self.tantos, Trucos::ReTruco)))
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
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) && self.players.is_accepting(player) {
            self.players.chalenges(player);
            Ok(Box::new(ValeCuatro::new(self.tantos, self.players)))
        } else {
            Err(self)
        }
    }

    fn tirar_carta(
        mut self: Box<Self>,
        player: &str,
        card: Carta,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        match self.players.tirar_carta(player, card) {
            Ok(_) => Ok(self),
            Err(_) => Err(self),
        }
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Ok(self.tantos)
    }

    fn valor_ronda(&self) -> Result<Trucos, &str> {
        Err("La ronda aun no a terminado.")
    }

    fn valid_commands(&self, player: &str) -> Vec<String> {
        let mut ret = vec!["irse_al_maso".to_string()];
        if self.players.is_turn(player) {
            ret.push("tirar_carta".to_string());
            if self.players.is_accepting(player) {
                ret.push("cantar_vale_cuatro".to_string());
            }
        }
        ret
    }
}
