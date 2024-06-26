use estados::{inicial::Inicial, Envidos, TrucoState};

#[derive(Debug, Clone, Copy)]
pub enum CantidadDeJugadores {
    Dos = 2,
    Cuatro = 4,
    Seis = 6,
}

pub struct TrucoStateMachine {
    internal_state: Box<dyn TrucoState>,
}

impl TrucoStateMachine {
    #[must_use]
    pub fn new(de_a: CantidadDeJugadores) -> Self {
        Self {
            internal_state: Box::new(Inicial::new(de_a)),
        }
    }

    pub fn irse_al_maso(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.irse_al_maso()?;
        Ok(())
    }

    pub fn cantar_quiero(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_quiero()?;
        Ok(())
    }

    pub fn cantar_no_quiero(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_no_quiero()?;
        Ok(())
    }

    pub fn cantar_envido(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_envido()?;
        Ok(())
    }

    pub fn cantar_real_envido(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_real_envido()?;
        Ok(())
    }

    pub fn cantar_falta_envido(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_falta_envido()?;
        Ok(())
    }

    pub fn cantar_truco(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_truco()?;
        Ok(())
    }

    pub fn cantar_re_truco(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_re_truco()?;
        Ok(())
    }

    pub fn cantar_vale_cuatro(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.cantar_vale_cuatro()?;
        Ok(())
    }

    pub fn tirar_carta(&mut self) -> Result<(), ()> {
        self.internal_state = self.internal_state.tirar_carta()?;
        Ok(())
    }

    pub fn tantos(&self) -> Result<Envidos, &str> {
        self.internal_state.tantos()
    }

    pub fn valor_ronda(&self) -> Result<u8, &str> {
        match self.internal_state.valor_ronda() {
            Ok(v) => Ok(v as u8),
            Err(e) => Err(e),
        }
    }
}

mod estados;
