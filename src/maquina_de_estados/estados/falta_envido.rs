use crate::{carta::Carta, equipos::Equipo};

use super::{nada::Nada, player_state::PlayersState, r#final::Final, Envidos, TrucoState, Trucos};

#[derive(Debug, Clone)]
pub struct FaltaEnvido {
    tantos: Envidos,
    players: PlayersState,
}

impl FaltaEnvido {
    pub fn new(tantos: Envidos, players: PlayersState) -> Self {
        Self { tantos, players }
    }
}

impl TrucoState for FaltaEnvido {
    fn irse_al_maso(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        let players_team = self.players.team(player);
        match players_team {
            Ok(players_team) => Ok(Box::new(Final::new(
                self.tantos,
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
            Ok(Box::new(Nada::new(Envidos::Falta, self.players)))
        } else {
            Err(self)
        }
    }

    fn cantar_no_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            Ok(Box::new(Nada::new(self.tantos + 1, self.players)))
        } else {
            Err(self)
        }
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
}
