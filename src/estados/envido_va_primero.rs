use super::{
    envido::Envido, falta_envido::FaltaEnvido, r#final::Final, re_truco::ReTruco,
    real_envido::RealEnvido, truco_querido::TrucoQuerido, Envidos, Players, TrucoState, Trucos,
};

#[derive(Debug, Clone)]
pub struct EnvidoVaPrimero {
    players: Players,
}

impl EnvidoVaPrimero {
    pub fn new(players: Players) -> Self {
        Self { players }
    }
}

impl TrucoState for EnvidoVaPrimero {
    fn irse_al_maso(self: Box<Self>) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Ok(Box::new(Final::new(Envidos::Value(0), Trucos::Simple)))
    }

    fn cantar_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_team(player) {
            Err(self)
        } else {
            Ok(Box::new(TrucoQuerido::new(Envidos::Value(0), self.players)))
        }
    }

    fn cantar_no_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_team(player) {
            Err(self)
        } else {
            Ok(Box::new(Final::new(Envidos::Value(0), Trucos::Simple)))
        }
    }

    fn cantar_envido(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_team(player) {
            Err(self)
        } else {
            Ok(Box::new(Envido::new(self.players)))
        }
    }

    fn cantar_real_envido(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_team(player) {
            Err(self)
        } else {
            Ok(Box::new(RealEnvido::new(Envidos::Value(0), self.players)))
        }
    }

    fn cantar_falta_envido(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_team(player) {
            Err(self)
        } else {
            Ok(Box::new(FaltaEnvido::new(Envidos::Value(0), self.players)))
        }
    }

    fn cantar_truco(self: Box<Self>, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn cantar_re_truco(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_team(player) {
            Err(self)
        } else {
            Ok(Box::new(ReTruco::new(Envidos::Value(0), self.players)))
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
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<Trucos, &str> {
        Err("La ronda aun no a terminado.")
    }
}
