use crate::{carta::Carta, equipos::Equipo};

use super::{
    player_state::PlayersState, r#final::Final, re_truco::ReTruco, Envidos, TrucoState, Trucos,
};

#[derive(Debug, Clone)]
pub struct TrucoQuerido {
    tantos: Envidos,
    players: PlayersState,
}

impl TrucoQuerido {
    pub fn new(tantos: Envidos, players: PlayersState) -> Self {
        Self { tantos, players }
    }
}

impl TrucoState for TrucoQuerido {
    fn irse_al_maso(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        let players_team = self.players.team(player);
        match players_team {
            Ok(players_team) => Ok(Box::new(Final::new(
                self.tantos,
                Trucos::Truco,
                Some(!players_team),
            ))),
            Err(_) => Err(self),
        }
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
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) && self.players.is_accepting(player) {
            self.players.chalenges(player);
            Ok(Box::new(ReTruco::new(self.tantos, self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_vale_cuatro(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn tirar_carta(
        mut self: Box<Self>,
        player: &str,
        card: Carta,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        match self.players.tirar_carta(player, card) {
            Ok(round_ending) => {
                if let Some(Some(sub_game_ending)) = round_ending {
                    Ok(Box::new(Final::new(
                        self.tantos,
                        Trucos::Simple,
                        sub_game_ending,
                    )))
                } else {
                    Ok(self)
                }
            }
            Err(()) => Err(self),
        }
    }

    fn tantos(&self) -> Result<Envidos, &'static str> {
        Ok(self.tantos)
    }

    fn valor_ronda(&self) -> Result<Trucos, &'static str> {
        Err("La ronda aun no a terminado.")
    }

    fn valid_commands(&self, player: &str) -> Vec<String> {
        let mut ret = vec!["irse_al_maso".to_string()];
        if self.players.is_turn(player) {
            ret.push("tirar_carta".to_string());
            if self.players.is_accepting(player) {
                ret.push("cantar_re_truco".to_string());
            }
        }
        ret
    }

    fn winner(&self) -> Result<Option<Equipo>, &'static str> {
        Err("La ronda aun no a terminado.")
    }

    fn turn(&self) -> Box<str> {
        self.players.turn()
    }
}
