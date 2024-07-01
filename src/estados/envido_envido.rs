use super::{
    falta_envido::FaltaEnvido, nada::Nada, r#final::Final, real_envido::RealEnvido, Envidos,
    Players, TrucoState, Trucos,
};

#[derive(Debug, Clone)]
pub struct EnvidoEnvido {
    players: Players,
}

impl EnvidoEnvido {
    pub fn new(players: Players) -> Self {
        Self { players }
    }
}

const VALOR_QUERIDO: u8 = 4;
const VALOR_NO_QUERIDO: u8 = 3;

impl TrucoState for EnvidoEnvido {
    fn irse_al_maso(self) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Ok(Box::new(Final::new(
            Envidos::Value(VALOR_NO_QUERIDO),
            Trucos::Simple,
        )))
    }

    fn cantar_quiero(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(Nada::new(Envidos::Value(VALOR_QUERIDO))))
        } else {
            Err(Box::new(self))
        }
    }

    fn cantar_no_quiero(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(Nada::new(Envidos::Value(VALOR_NO_QUERIDO))))
        } else {
            Err(Box::new(self))
        }
    }

    fn cantar_envido(self, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(Box::new(self))
    }

    fn cantar_real_envido(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(RealEnvido::new(Envidos::Value(VALOR_QUERIDO))))
        } else {
            Err(Box::new(self))
        }
    }

    fn cantar_falta_envido(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(FaltaEnvido::new(Envidos::Value(VALOR_QUERIDO))))
        } else {
            Err(Box::new(self))
        }
    }

    fn cantar_truco(self, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(Box::new(self))
    }

    fn cantar_re_truco(self, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(Box::new(self))
    }

    fn cantar_vale_cuatro(self, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(Box::new(self))
    }

    fn tirar_carta(self, _: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(Box::new(self))
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<Trucos, &str> {
        Err("La ronda aun no a terminado.")
    }
}
