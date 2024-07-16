use crate::juego::TrucoBuilder;

#[test]
fn partida() {
    let _game = TrucoBuilder::new()
        .add_player("A".to_string())
        .add_player("B".to_string())
        .hasta(15)
        .build();
}
