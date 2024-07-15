use crate::juego::TrucoBuilder;

#[test]
fn partida() {
    let mut game_builder = TrucoBuilder::new();
    game_builder.add_player("A".to_string());
    game_builder.add_player("B".to_string());
    let _game = game_builder.build();
}
