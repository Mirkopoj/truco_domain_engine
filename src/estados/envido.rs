use super::{
    envido_envido::EnvidoEnvido, falta_envido::FaltaEnvido, nada::Nada, r#final::Final,
    real_envido::RealEnvido, Envidos, Players, TrucoState, Trucos,
};

#[derive(Debug, Clone)]
pub struct Envido {
    players: Players,
}

impl Envido {
    pub fn new(players: Players) -> Self {
        Self { players }
    }
}

const VALOR_QUERIDO: u8 = 2;
const VALOR_NO_QUERIDO: u8 = 1;

impl TrucoState for Envido {
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

    fn cantar_envido(self, player: &str) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_turn(player) {
            Ok(Box::new(EnvidoEnvido::new(self.players)))
        } else {
            Err(Box::new(self))
        }
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

    fn tantos(&self) -> std::result::Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> std::result::Result<Trucos, &str> {
        Err("La ronda aun no a terminado.")
    }
}
