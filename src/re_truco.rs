use crate::{
    r#final::Final, re_truco_querido::ReTrucoQuerido, vale_cuatro::ValeCuatro, Envidos, TrucoState,
    Trucos,
};

#[derive(Debug, Clone, Copy)]
pub struct ReTruco {
    tantos: Envidos,
}

impl ReTruco {
    pub fn new(tantos: Envidos) -> Self {
        Self { tantos }
    }
}

impl TrucoState for ReTruco {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(self.tantos, Trucos::Truco)))
    }

    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(ReTrucoQuerido::new(self.tantos)))
    }

    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(self.tantos, Trucos::Truco)))
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
        Ok(Box::new(ValeCuatro::new(self.tantos)))
    }

    fn tirar_carta(&mut self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(ReTrucoQuerido::new(self.tantos)))
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Ok(self.tantos)
    }

    fn valor_ronda(&self) -> Result<u8, &str> {
        Err("La ronda aun no a terminado.")
    }
}
