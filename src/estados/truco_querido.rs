use super::{r#final::Final, re_truco::ReTruco, Envidos, TrucoState, Trucos};

#[derive(Debug, Clone, Copy)]
pub struct TrucoQuerido{
    tantos: Envidos,
}

impl TrucoQuerido {
    pub fn new(tantos: Envidos) -> Self {
        Self { tantos }
    }
}

impl TrucoState for TrucoQuerido {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(self.tantos, Trucos::Truco)))
    }

    fn cantar_quiero(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_no_quiero(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_envido(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_real_envido(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_falta_envido(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_truco(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_re_truco(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(ReTruco::new(self.tantos)))
    }

    fn cantar_vale_cuatro(&self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn tirar_carta(&mut self, player: &str) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(*self))
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Ok(self.tantos)
    }

    fn valor_ronda(&self) -> Result<Trucos, &str> {
        Err("La ronda aun no a terminado.")
    }
}
