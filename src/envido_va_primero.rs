use crate::{
    envido::Envido, falta_envido::FaltaEnvido, r#final::Final, re_truco::ReTruco,
    real_envido::RealEnvido, truco_querido::TrucoQuerido, Envidos, TrucoState, Trucos,
};

#[derive(Debug, Clone, Copy)]
pub struct EnvidoVaPrimero;

impl TrucoState for EnvidoVaPrimero {
    fn irse_al_maso(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(Envidos::Value(0), Trucos::Simple)))
    }

    fn cantar_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(TrucoQuerido::new(Envidos::Value(0))))
    }

    fn cantar_no_quiero(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(Final::new(Envidos::Value(0), Trucos::Simple)))
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
        Err(())
    }

    fn cantar_re_truco(&self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(ReTruco::new(Envidos::Value(0))))
    }

    fn cantar_vale_cuatro(&self) -> Result<Box<dyn TrucoState>, ()> {
        Err(())
    }

    fn tirar_carta(&mut self) -> Result<Box<dyn TrucoState>, ()> {
        Ok(Box::new(TrucoQuerido::new(Envidos::Value(0))))
    }

    fn tantos(&self) -> Result<Envidos, &str> {
        Err("El envido aun no se termina de cantar.")
    }

    fn valor_ronda(&self) -> Result<u8, &str> {
        Err("La ronda aun no a terminado.")
    }
}
