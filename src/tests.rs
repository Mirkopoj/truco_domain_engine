use crate::{CantidadDeJugadores, Envidos, TrucoStateMachine};

#[test]
fn inicial() {
    let game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert!(game.tantos().is_err());
    assert!(game.valor_ronda().is_err());
}

#[test]
fn mazo() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.irse_al_maso());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(1));
}

#[test]
fn truco() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.irse_al_maso());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(2));
}

#[test]
fn re_truco() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn re_truco_con_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.irse_al_maso());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(3));
}

#[test]
fn vale_cuatro() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_vale_cuatro());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.irse_al_maso());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(4));
}

#[test]
fn vale_cuatro_con_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_vale_cuatro());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.irse_al_maso());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(4));
}

#[test]
fn envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(2)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn real_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn falta_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(4)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_real_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(5)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido_real_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(7)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_falta_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn real_envido_falta_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_falta_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_real_envido_falta_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_real_envido_falta_envido() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn truco_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(1));
}

#[test]
fn re_truco_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(2));
}

#[test]
fn re_truco_con_quiero_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(2));
}

#[test]
fn vale_cuatro_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_vale_cuatro());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(3));
}

#[test]
fn vale_cuatro_con_quiero_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_vale_cuatro());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(3));
}

#[test]
fn envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(1)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn real_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(1)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn falta_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(1)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_real_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido_real_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(5)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn real_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(4)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(5)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_real_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(game.tantos(), Ok(Envidos::Value(6)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido_real_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_no_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(game.tantos(), Ok(Envidos::Value(8)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn card_allowance_con_quieros() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_vale_cuatro());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.tirar_carta());
}

#[test]
fn card_allowance() {
    let mut game = TrucoStateMachine::new(CantidadDeJugadores::Dos);
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_real_envido());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_falta_envido());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Err(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_truco());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.cantar_re_truco());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Ok(()), game.cantar_vale_cuatro());
    //CondicionDeJugador::new(true, Equipo::Nosotros),
    assert_eq!(Ok(()), game.tirar_carta());
    assert_eq!(Err(()), game.cantar_quiero());
    //CondicionDeJugador::new(true, Equipo::Ellos),
    assert_eq!(Ok(()), game.tirar_carta());
}
