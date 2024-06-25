use crate::{
    envido::Envido, envido_va_primero::EnvidoVaPrimero, falta_envido::FaltaEnvido, nada::Nada,
    r#final::Final, real_envido::RealEnvido, CantidadDeJugadores, Envidos, TrucoState, Trucos,
};

#[derive(Debug, Clone, Copy)]
pub struct Inicial {
    cont: u8,
    de_a: CantidadDeJugadores,
}

impl Inicial {
    pub fn new(de_a: CantidadDeJugadores) -> Self {
        Self { cont: 0, de_a }
    }
}

impl TrucoState for Inicial {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(Envidos::Value(0), Trucos::Simple)))
    }

    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Envido))
    }

    fn cantar_real_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(RealEnvido::new(Envidos::Value(0))))
    }

    fn cantar_falta_envido(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(FaltaEnvido::new(Envidos::Value(0))))
    }

    fn cantar_truco(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(EnvidoVaPrimero))
    }

    fn cantar_re_truco(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn cantar_vale_cuatro(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn tirar_carta(&mut self) -> Result<Box<dyn TrucoState>, ()> {
        self.cont += 1;
        if self.cont == self.de_a as u8 {
            Ok(Box::new(Nada::new(Envidos::Value(0))))
        } else {
            Ok(Box::new(*self))
        }
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<u8, &str> {
        Err("La ronda aun no a terminado.")
    }
}
