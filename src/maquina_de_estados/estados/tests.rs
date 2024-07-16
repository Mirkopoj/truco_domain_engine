use crate::{
    carta::{Carta, Numero, Palo},
    equipos::Equipo,
    maquina_de_estados::{estados::Envidos, TrucoStateMachineBuilder},
};

#[test]
fn inicial() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.tantos().is_err());
    assert!(game.valor_ronda().is_err());
}

#[test]
fn mazo() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.irse_al_maso("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(1));
}

#[test]
fn truco() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game.irse_al_maso("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(2));
}

#[test]
fn re_truco() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_re_truco("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn re_truco_con_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Dos, Palo::Oro))
        .is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game.irse_al_maso("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(3));
}

#[test]
fn vale_cuatro() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game.cantar_vale_cuatro("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game.irse_al_maso("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(4));
}

#[test]
fn vale_cuatro_con_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Dos, Palo::Oro))
        .is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Cuatro, Palo::Oro))
        .is_ok());
    assert!(game.cantar_vale_cuatro("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game.irse_al_maso("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(4));
}

#[test]
fn envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(2)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn real_envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn falta_envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(4)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_real_envido() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(5)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido_real_envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(7)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_falta_envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_falta_envido("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn real_envido_falta_envido() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_real_envido("A").is_ok());
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_falta_envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_real_envido_falta_envido() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_falta_envido("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_real_envido_falta_envido() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_falta_envido("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Falta));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn truco_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_no_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(1));
}

#[test]
fn re_truco_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_re_truco("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(2));
}

#[test]
fn re_truco_con_quiero_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Dos, Palo::Oro))
        .is_ok());
    assert!(game.cantar_re_truco("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(2));
}

#[test]
fn vale_cuatro_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game.cantar_vale_cuatro("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(3));
}

#[test]
fn vale_cuatro_con_quiero_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game.cantar_re_truco("B").is_err());
    assert!(game.cantar_re_truco("A").is_err());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Dos, Palo::Oro))
        .is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Cuatro, Palo::Oro))
        .is_ok());
    assert!(game.cantar_vale_cuatro("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(0)));
    assert_eq!(game.valor_ronda(), Ok(3));
}

#[test]
fn envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(1)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn real_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(1)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn falta_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(1)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_no_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_real_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido_real_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(5)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_falta_envido("A").is_ok());
    assert!(game.cantar_no_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(3)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn real_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game.cantar_real_envido("A").is_ok());
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(4)));
    assert!(game.valor_ronda().is_err());
}

#[test]
fn envido_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(5)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_real_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_real_envido("A").is_ok());
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game.cantar_no_quiero("A").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(6)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn envido_envido_real_envido_falta_envido_no_quiero() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game.cantar_envido("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game.cantar_falta_envido("A").is_ok());
    assert!(game.cantar_no_quiero("B").is_ok());
    assert_eq!(game.tantos(), Ok(Envidos::Value(8)));
    assert_eq!(game.valor_ronda(), Err("La ronda aun no a terminado."));
}

#[test]
fn card_allowance_con_quieros() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Dos, Palo::Oro))
        .is_ok());
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Tres, Palo::Oro))
        .is_err());
    assert!(game.cantar_envido("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Cuatro, Palo::Oro))
        .is_err());
    assert!(game.cantar_real_envido("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Cinco, Palo::Oro))
        .is_err());
    assert!(game.cantar_falta_envido("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Seis, Palo::Oro))
        .is_err());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Sota, Palo::Oro))
        .is_ok());
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Siete, Palo::Oro))
        .is_ok());
    assert!(game.cantar_re_truco("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Siete, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Cuatro, Palo::Espada))
        .is_ok());
    assert!(game.cantar_vale_cuatro("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
}

#[test]
fn card_allowance() {
    let mut game = TrucoStateMachineBuilder::new().add_player("B".to_string()).add_player("A".to_string()).build();
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Dos, Palo::Oro))
        .is_ok());
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_envido("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Tres, Palo::Oro))
        .is_err());
    assert!(game.cantar_envido("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Cuatro, Palo::Oro))
        .is_err());
    assert!(game.cantar_real_envido("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Cinco, Palo::Oro))
        .is_err());
    assert!(game.cantar_falta_envido("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Seis, Palo::Oro))
        .is_err());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Sota, Palo::Oro))
        .is_ok());
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Sota, Palo::Oro))
        .is_err());
    assert!(game.cantar_vale_cuatro("B").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Caballo, Palo::Oro))
        .is_err());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
}

#[test]
fn no_repeats() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Oro))
        .is_err());
}

#[test]
fn no_repeats_between_rounds() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_err());
}

#[test]
fn winner_goes_first() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Ancho, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Basto))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Basto))
        .is_ok());
}

#[test]
fn pardas() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Basto))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Ancho, Palo::Espada))
        .is_ok());
}

#[test]
fn aba_winner_a() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Ancho, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Basto))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Basto))
        .is_ok());
    assert!(game.valor_ronda().is_ok());
    assert!(matches!(game.winner(), Ok(Some(Equipo::Nosotros))));
}

#[test]
fn aa_winner_a() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game.valor_ronda().is_ok());
    assert!(matches!(game.winner(), Ok(Some(Equipo::Nosotros))));
}

#[test]
fn ppb_winner_b() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Espada))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Ancho, Palo::Copa))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Basto))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Ancho, Palo::Espada))
        .is_ok());
    assert!(game.valor_ronda().is_ok());
    assert!(matches!(game.winner(), Ok(Some(Equipo::Ellos))));
}

#[test]
fn ppp_winner_none() {
    let mut game = TrucoStateMachineBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .build();
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Copa))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Ancho, Palo::Oro))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Ancho, Palo::Copa))
        .is_ok());
    assert!(game
        .tirar_carta("A", Carta::new(Numero::Rey, Palo::Basto))
        .is_ok());
    assert!(game
        .tirar_carta("B", Carta::new(Numero::Rey, Palo::Espada))
        .is_ok());
    assert!(game.valor_ronda().is_ok());
    assert!(matches!(game.winner(), Ok(None)));
}
