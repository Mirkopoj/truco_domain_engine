use crate::{nada::Nada, r#final::Final, Envidos, TrucoState, Trucos};

#[derive(Debug, Clone, Copy)]
pub struct FaltaEnvido {
    tantos: Envidos,
}

impl FaltaEnvido {
    pub fn new(tantos: Envidos) -> Self {
        Self { tantos }
    }
}

impl TrucoState for FaltaEnvido {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(self.tantos, Trucos::Simple)))
    }

    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Nada::new(Envidos::Falta)))
    }

    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Nada::new(self.tantos + 1)))
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
        Err(())
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<u8, &str> {
        Err("La ronda aun no a terminado.")
    }
}
