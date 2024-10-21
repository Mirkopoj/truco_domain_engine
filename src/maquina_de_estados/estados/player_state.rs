use crate::{carta::Carta, equipos::Equipo};

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
    prev_cards: Vec<Carta>,
    evens_accepting: bool,
    round_winners: Vec<Option<Equipo>>,
}

type RoundEnding = Option<SubGameEnded>;
type SubGameEnded = Option<SubGameEndedWinner>;
type SubGameEndedWinner = Option<Equipo>;

impl PlayersState {
    pub(super) fn new(players: Vec<String>, mano: usize) -> Self {
        Self {
            cont: mano,
            initial: mano,
            players: players
                .into_iter()
                .map(|n| PlayerThrownCard {
                    name: n,
                    card: None,
                })
                .collect(),
            prev_cards: Vec::new(),
            evens_accepting: false,
            round_winners: Vec::new(),
        }
    }

    pub(super) fn is_turn(&self, player: &str) -> bool {
        player == self.players[self.cont].name
    }

    /// Returns true if that was the last player in
    /// the round and the counter went back to 0
    fn next_player(&mut self) -> bool {
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

    fn was_played_peviosly(&self, card: Carta) -> bool {
        self.players
            .iter()
            .filter_map(|p| p.card)
            .chain(self.prev_cards.iter().copied())
            .any(|c| c == card)
    }

    fn best_card(&self) -> (usize, Carta) {
        self.players
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, p)| p.card.map(|c| (i, c)))
            .max_by(|(_, x), (_, y)| x.valor_juego().cmp(&y.valor_juego()))
            .expect("Players Shoudn't be empty")
    }

    fn round_has_winner(&self, best_card: Carta) -> bool {
        self.players
            .iter()
            .filter_map(|p| p.card)
            .filter(|c| c.valor_juego() == best_card.valor_juego())
            .count()
            == 1
    }

    //Pardas 3ra gana 1ra probablemente está mal.
    fn sub_game_ended(&self) -> SubGameEnded {
        if self.round_winners.len() < 2 {
            return None;
        }
        let wins = self.round_winners.iter().filter_map(|&w| w);
        let ellos = wins.clone().filter(|&w| matches!(w, Equipo::Ellos)).count();
        let nosotros = wins.filter(|&w| matches!(w, Equipo::Nosotros)).count();
        match ellos.cmp(&nosotros) {
            std::cmp::Ordering::Less => Some(Some(Equipo::Nosotros)),
            std::cmp::Ordering::Equal => {
                if self.round_winners.len() > 2 {
                    Some(None)
                } else {
                    None
                }
            }
            std::cmp::Ordering::Greater => Some(Some(Equipo::Ellos)),
        }
    }

    pub(super) fn tirar_carta(&mut self, player: &str, card: Carta) -> Result<RoundEnding, ()> {
        if !self.is_turn(player) || self.was_played_peviosly(card) {
            println!(
                "{player}({}) called in turn of {}({}), carta {card} ({}), was played prev {}",
                player.len(),
                self.players[self.cont].name,
                self.players[self.cont].name.len(),
                player == self.players[self.cont].name,
                self.was_played_peviosly(card)
            );
            return Err(());
        }
        self.players[self.cont].card = Some(card);
        let ret = if self.next_player() {
            let (best_cards_position, best_card) = self.best_card();
            let winner = if self.round_has_winner(best_card) {
                Some(Equipo::from(best_cards_position))
            } else {
                None
            };
            self.round_winners.push(winner);
            self.initial = best_cards_position;
            self.cont = self.initial;
            self.players.iter_mut().for_each(|player| {
                self.prev_cards.push(
                    player
                        .card
                        .take()
                        .expect("All players should allready have their cards thrown"),
                );
            });
            Some(self.sub_game_ended())
        } else {
            None
        };
        println!("Now is turn of {}", self.players[self.cont].name);
        Ok(ret)
    }

    pub(super) fn team(&self, player: &str) -> Result<Equipo, &str> {
        if let Some(index) = self.players.iter().position(|p| p.name == player) {
            Ok(Equipo::from(index))
        } else {
            Err("Non Existing player")
        }
    }

    pub(super) fn turn(&self) -> Box<str> {
        self.players[self.cont].name.clone().into()
    }

    pub(super) fn accepting(&self) -> Box<str> {
        self.players
            .iter()
            .filter_map(|p| {
                if self.is_accepting(&p.name) {
                    Some(&p.name)
                } else {
                    None
                }
            })
            .fold(String::new(), |a, b| a + b + ", ")
            .into()
    }
}
