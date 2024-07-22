use crate::{carta::Carta, equipos::Equipo};

use super::{
    envido::Envido, envido_va_primero::EnvidoVaPrimero, falta_envido::FaltaEnvido, nada::Nada,
    player_state::PlayersState, r#final::Final, real_envido::RealEnvido, Envidos, TrucoState,
    Trucos,
};

#[derive(Debug, Clone)]
pub struct Inicial {
    players: PlayersState,
}

impl Inicial {
    pub fn new(players: Vec<String>, mano: usize) -> Self {
        Self {
            players: PlayersState::new(players, mano),
        }
    }
}

impl TrucoState for Inicial {
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

    fn cantar_quiero(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_no_quiero(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_envido(
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
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
        if self.players.is_turn(player) {
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
        if self.players.is_turn(player) {
            self.players.chalenges(player);
            Ok(Box::new(FaltaEnvido::new(Envidos::Value(0), self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_truco(
        mut self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            self.players.chalenges(player);
            Ok(Box::new(EnvidoVaPrimero::new(self.players)))
        } else {
            Err(self)
        }
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
        mut self: Box<Self>,
        player: &str,
        card: Carta,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        match self.players.tirar_carta(player, card) {
            Ok(end_of_round) => {
                if end_of_round.is_some() {
                    Ok(Box::new(Nada::new(Envidos::Value(0), self.players)))
                } else {
                    Ok(self)
                }
            }
            Err(()) => Err(self),
        }
    }

    fn tantos(&self) -> Result<Envidos, &'static str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<Trucos, &'static str> {
        Err("La ronda aun no a terminado.")
    }

    fn valid_commands(&self, player: &str) -> Vec<String> {
        let mut ret = vec!["irse_al_maso".to_string()];
        if self.players.is_turn(player) {
            ret.push("cantar_envido".to_string());
            ret.push("cantar_real_envido".to_string());
            ret.push("cantar_falta_envido".to_string());
            ret.push("cantar_truco".to_string());
            ret.push("tirar_carta".to_string());
        }
        ret
    }

    fn winner(&self) -> Result<Option<Equipo>, &'static str> {
        Err("La ronda aun no a terminado.")
    }
}
