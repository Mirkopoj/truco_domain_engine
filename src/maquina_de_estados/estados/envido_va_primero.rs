use crate::{carta::Carta, equipos::Equipo};

use super::{
    envido::Envido, falta_envido::FaltaEnvido, player_state::PlayersState, r#final::Final,
    re_truco::ReTruco, real_envido::RealEnvido, truco_querido::TrucoQuerido, Envidos, TrucoState,
    Trucos,
};

#[derive(Debug, Clone)]
pub struct EnvidoVaPrimero {
    players: PlayersState,
}

impl EnvidoVaPrimero {
    pub fn new(players: PlayersState) -> Self {
        Self { players }
    }
}

impl TrucoState for EnvidoVaPrimero {
    fn irse_al_maso(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        let players_team = self.players.team(player);
        match players_team {
            Ok(players_team) => Ok(Box::new(Final::new(
                Envidos::Value(0),
                Trucos::Simple,
                Some(!players_team),
            ))),
            Err(_) => Err(self),
        }
    }

    fn cantar_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            Ok(Box::new(TrucoQuerido::new(Envidos::Value(0), self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_no_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            let players_team = self.players.team(player);
            match players_team {
                Ok(players_team) => Ok(Box::new(Final::new(
                    Envidos::Value(0),
                    Trucos::Simple,
                    Some(!players_team),
                ))),
                Err(_) => Err(self),
            }
        } else {
            Err(self)
        }
    }

    fn cantar_envido(
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            self.players.chalenges(player);
            Ok(Box::new(Envido::new(self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_real_envido(
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            self.players.chalenges(player);
            Ok(Box::new(RealEnvido::new(Envidos::Value(0), self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_falta_envido(
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            self.players.chalenges(player);
            Ok(Box::new(FaltaEnvido::new(Envidos::Value(0), self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_truco(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_re_truco(
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            self.players.chalenges(player);
            Ok(Box::new(ReTruco::new(Envidos::Value(0), self.players)))
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
        self: Box<Self>,
        _: &str,
        _: Carta,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn tantos(&self) -> Result<Envidos, &'static str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<Trucos, &'static str> {
        Err("La ronda aun no a terminado.")
    }

    fn valid_commands(&self, player: &str) -> Vec<String> {
        let mut ret = vec!["irse_al_maso".to_string()];
        if self.players.is_accepting(player) {
            ret.push("cantar_quiero".to_string());
            ret.push("cantar_no_quiero".to_string());
            ret.push("cantar_envido".to_string());
            ret.push("cantar_real_envido".to_string());
            ret.push("cantar_falta_envido".to_string());
            ret.push("cantar_re_truco".to_string());
        }
        ret
    }

    fn winner(&self) -> Result<Option<Equipo>, &'static str> {
        Err("La ronda aun no a terminado.")
    }

    fn turn(&self) -> Box<str> {
        self.players.accepting()
    }
}
