use crate::{equipos::Equipo, juego::TrucoBuilder};

#[test]
fn partida() {
    let mut game = TrucoBuilder::new()
        .add_player("A")
        .add_player("B")
        .hasta(15)
        .build();
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_re_truco("B").is_ok());
    assert!(game.cantar_vale_cuatro("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game.irse_al_maso("A").is_ok());
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game.cantar_vale_cuatro("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game.tirar_carta("B", 0).is_ok());
    assert!(game.irse_al_maso("A").is_ok());
    assert!(game.cantar_truco("A").is_ok());
    assert!(game.cantar_re_truco("B").is_ok());
    assert!(game.cantar_vale_cuatro("A").is_ok());
    assert!(game.cantar_quiero("B").is_ok());
    assert!(game.irse_al_maso("A").is_ok());
    assert!(game.cantar_truco("B").is_ok());
    assert!(game.cantar_re_truco("A").is_ok());
    assert!(game.cantar_vale_cuatro("B").is_ok());
    assert!(game.cantar_quiero("A").is_ok());
    assert!(game.tirar_carta("B", 0).is_ok());
    assert!(game.ganador().is_none());
    assert!(game.irse_al_maso("A").is_ok());
    assert!(game.irse_al_maso("B").is_err());
    assert!(game.irse_al_maso("A").is_err());
    assert!(game.terminado());
    assert_eq!(game.ganador(), Some(Equipo::Ellos));
}

#[test]
fn tirar() {
    let mut game = TrucoBuilder::new()
        .add_player("A")
        .add_player("B")
        .hasta(15)
        .build();
    assert!(game.tirar_carta("B", 0).is_err());
    assert!(game.tirar_carta("A", 0).is_ok());
    assert!(game.tirar_carta("B", 0).is_ok());
    assert!(game.tirar_carta("A", 0).is_err());
    assert!(game.tirar_carta("B", 0).is_err());
}
