use super::{
    envido_envido::EnvidoEnvido, falta_envido::FaltaEnvido, nada::Nada, r#final::Final,
    real_envido::RealEnvido, Envidos, PlayersState, TrucoState, Trucos,
};

#[derive(Debug, Clone)]
pub struct Envido {
    players: PlayersState,
}

impl Envido {
    pub fn new(players: PlayersState) -> Self {
        Self { players }
    }
}

const VALOR_QUERIDO: u8 = 2;
const VALOR_NO_QUERIDO: u8 = 1;

impl TrucoState for Envido {
    fn irse_al_maso(self: Box<Self>) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Ok(Box::new(Final::new(
            Envidos::Value(VALOR_NO_QUERIDO),
            Trucos::Simple,
        )))
    }

    fn cantar_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            Ok(Box::new(Nada::new(
                Envidos::Value(VALOR_QUERIDO),
                self.players,
            )))
        } else {
            Err(self)
        }
    }

    fn cantar_no_quiero(
        self: Box<Self>,
        player: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        if self.players.is_accepting(player) {
            Ok(Box::new(Nada::new(
                Envidos::Value(VALOR_NO_QUERIDO),
                self.players,
            )))
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
            Ok(Box::new(EnvidoEnvido::new(self.players)))
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
            Ok(Box::new(RealEnvido::new(
                Envidos::Value(VALOR_QUERIDO),
                self.players,
            )))
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
            Ok(Box::new(FaltaEnvido::new(
                Envidos::Value(VALOR_QUERIDO),
                self.players,
            )))
        } else {
            Err(self)
        }
    }

    fn cantar_truco(
        self: Box<Self>,
        _: &str,
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
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
    ) -> Result<Box<dyn TrucoState>, Box<dyn TrucoState>> {
        Err(self)
    }

    fn tantos(&self) -> std::result::Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> std::result::Result<Trucos, &str> {
        Err("La ronda aun no a terminado.")
    }
}
