use super::{r#final::Final, Envidos, TrucoState, Trucos};

#[derive(Debug, Clone, Copy)]
pub struct ValeCuatroQuerido{
    tantos: Envidos,
}

impl ValeCuatroQuerido {
    pub fn new(tantos: Envidos) -> Self {
        Self { tantos }
    }
}

impl TrucoState for ValeCuatroQuerido {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(self.tantos, Trucos::ValeCuatro)))
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
        Err(())
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
