use crate::carta::Carta;

#[derive(Debug, Clone)]
struct PlayerThrownCard {
    name: String,
    card: Option<Carta>,
}

#[derive(Debug, Clone)]
pub(super) struct PlayersState {
    cont: usize,
    initial: usize,
    players: Vec<PlayerThrownCard>,
    evens_accepting: bool,
}

impl PlayersState {
    pub(super) fn new(players: Vec<String>) -> Self {
        Self {
            cont: 0,
            initial: 0,
            players: players
                .into_iter()
                .map(|n| PlayerThrownCard {
                    name: n,
                    card: None,
                })
                .collect(),
            evens_accepting: false,
        }
    }

    pub(super) fn is_turn(&self, player: &str) -> bool {
        player == self.players[self.cont].name
    }

    /// Returns true if that was the last player in
    /// the round and the counter went back to 0
    pub(super) fn next_player(&mut self) -> bool {
        self.cont += 1;
        self.cont %= self.players.len();
        self.cont == self.initial
    }

    pub(super) fn is_accepting(&self, player: &str) -> bool {
        if let Some(index) = self.players.iter().position(|p| p.name == player) {
            (index % 2 == 0) == self.evens_accepting
        } else {
            false
        }
    }

    pub(super) fn chalenges(&mut self, player: &str) {
        if let Some(index) = self.players.iter().position(|p| p.name == player) {
            self.evens_accepting = index % 2 != 0;
        }
    }

    pub(super) fn tirar_carta(&mut self, player: &str, card: Carta) -> Result<bool, ()> {
        if !self.is_turn(player) {
            return Err(());
        }
        self.players[self.cont].card = Some(card);
        let ret = self.next_player();
        if ret {
            self.initial = self
                .players
                .iter()
                .map(|p| {
                    p.card
                        .expect("All players should allready have their cards thrown")
                })
                .enumerate()
                .max_by(|(_, x), (_, y)| x.valor_juego().cmp(&y.valor_juego()))
                .expect("Players Shoudn't be empty")
                .0;
            self.cont = self.initial;
            self.players.iter_mut().for_each(|player| {
                player.card = None;
            });
        }
        Ok(ret)
    }
}
