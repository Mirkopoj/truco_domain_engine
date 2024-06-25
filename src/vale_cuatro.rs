use crate::{r#final::Final, vale_cuatro_querido::ValeCuatroQuerido, Envidos, TrucoState, Trucos};

#[derive(Debug, Clone, Copy)]
pub struct ValeCuatro{
    tantos: Envidos,
}

impl ValeCuatro {
    pub fn new(tantos: Envidos) -> Self {
        Self { tantos }
    }
}

impl TrucoState for ValeCuatro {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(self.tantos, Trucos::ReTruco)))
    }

    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(ValeCuatroQuerido::new(self.tantos)))
    }

    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(self.tantos, Trucos::ReTruco)))
    }

    fn cantar_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_real_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_falta_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_truco(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_re_truco(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_vale_cuatro(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn tirar_carta(&mut self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(ValeCuatroQuerido::new(self.tantos)))
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Ok(self.tantos)
    }

    fn valor_ronda(&self) -> Result<u8, &str> {
        Err("La ronda aun no a terminado.")
    }
}
