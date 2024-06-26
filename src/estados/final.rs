use super::{Envidos, TrucoState, Trucos};

#[derive(Debug, Clone, Copy)]
pub struct Final {
    tantos: Envidos,
    valor_ronda: Trucos,
}

impl Final {
    pub fn new(tantos: Envidos, valor_ronda: Trucos) -> Self {
        Self {
            tantos,
            valor_ronda,
        }
    }
}

impl TrucoState for Final {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
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
        Ok(self.tantos)
    }

    fn valor_ronda(&self) -> Result<Trucos, &str> {
        Ok(self.valor_ronda)
    }
}
