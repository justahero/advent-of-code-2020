use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq)]
struct Deck {
    pub player_id: u64,
    pub cards: Vec<u64>,
}

impl Deck {
    /// Parses the name and list of cards
    pub fn parse(content: &str, player_id: u64) -> anyhow::Result<Self> {
        // parse player name
        let lines = content
            .lines()
            .map(str::trim)
            .collect::<Vec<_>>();

        let cards = lines[1..]
            .iter()
            .map(|&v| v.parse::<u64>())
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(Self {
            player_id,
            cards,
        })
    }

    /// Show the top card of the deck
    pub fn draw_card(&mut self) -> u64 {
        self.cards.remove(0)
    }

    /// Returns the number of remaining cards
    pub fn remaining(&self) -> u64 {
        self.cards.len() as u64
    }

    /// Returns true if deck has no cards
    pub fn empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Draw top card from other player, put on to bottom of card deck
    pub fn put_cards(&mut self, cards: &[u64])  {
        self.cards.append(&mut cards.to_vec());
    }

    /// Returns the score of this deck
    pub fn score(&self) -> usize {
        let length = self.cards.len();
        self.cards
            .iter()
            .enumerate()
            .map(|(index, &v)| v as usize * (length - index))
            .sum()
    }

    /// Creates a copy of this deck with the number of cards
    pub fn copy(&self, num_cards: u64) -> Deck {
        Self {
            player_id: self.player_id,
            cards: self.cards[..num_cards as usize].to_vec(),
        }
    }
}

/// Plays the game between players
fn play_game_1(mut player1: Deck, mut player2: Deck) -> Deck {
    loop {
        if player1.empty() {
            return player2;
        }
        if player2.empty() {
            return player1;
        }

        play_round(&mut player1, &mut player2);
    }
}

struct GameRecursive {
    pub number: u64,
    pub player1: Deck,
    pub player2: Deck,
    pub previous: Vec<(Deck, Deck)>,
}

impl GameRecursive {
    pub fn new(game_number: u64, player1: Deck, player2: Deck) -> Self {
        Self {
            number: game_number,
            player1,
            player2,
            previous: Vec::new(),
        }
    }

    /// Play the round until game finishes
    pub fn play(&mut self) -> &Deck {
        loop {
            if self.player1.empty() {
                return &self.player2;
            }
            if self.player2.empty() {
                return &self.player1;
            }

            // first check if there was a previous round
            if self.is_previous_round() {
                return &self.player1;
            }

            // add player decks to seen previous rounds
            self.mark_as_previous();

            self.next_round();
        }
    }

    /// Checks if there was a previous game
    fn is_previous_round(&self) -> bool {
        self.previous.contains(&(self.player1.clone(), self.player2.clone()))
    }

    /// Mark this current decks combination to seen list.
    fn mark_as_previous(&mut self) {
        self.previous.push((self.player1.clone(), self.player2.clone()));
    }

    /// Play a single round
    fn next_round(&mut self) {
        let top_card_1 = self.player1.draw_card();
        let top_card_2 = self.player2.draw_card();

        // both players have enough remaining cards, start a new game
        if top_card_1 <= self.player1.remaining() && top_card_2 <= self.player2.remaining() {

            let mut new_game = GameRecursive::new(
                self.number + 1,
                self.player1.copy(top_card_1),
                self.player2.copy(top_card_2),
            );

            if new_game.play().player_id == self.player1.player_id {
                self.player1.put_cards(&[top_card_1, top_card_2]);
            } else {
                self.player2.put_cards(&[top_card_2, top_card_1]);
            }
        } else if top_card_1 > top_card_2 {
            self.player1.put_cards(&[top_card_1, top_card_2]);
        } else {
            self.player2.put_cards(&[top_card_2, top_card_1]);
        }
    }
}

/// Play a single round, transfers cards accordingly, returns player who won
fn play_round(player1: &mut Deck, player2: &mut Deck) -> u64 {
    let top_card_1 = player1.draw_card();
    let top_card_2 = player2.draw_card();

    if top_card_1 > top_card_2 {
        player1.put_cards(&[top_card_1, top_card_2]);
        0
    } else {
        player2.put_cards(&[top_card_2, top_card_1]);
        1
    }
}

/// Parses the text content into two decks of cards
fn parse_decks(content: &str) -> anyhow::Result<(Deck, Deck)> {
    let parts = content
        .split("\n\n");

    let decks = parts
        .enumerate()
        .map(|(index, item)| Deck::parse(item, index as u64))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok((decks[0].clone(), decks[1].clone()))
}

fn main() -> anyhow::Result<()> {
    let (player1, player2) = parse_decks(include_str!("cards.txt"))?;
    let winner = play_game_1(player1.clone(), player2.clone());

    dbg!(winner.score());

    let mut game = GameRecursive::new(1, player1, player2);
    let winner = game.play();

    dbg!(winner.score());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{GameRecursive, parse_decks, play_game_1, play_round};

    const CARDS: &str = r#"Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10
    "#;

    #[test]
    fn test_parse_decks() {
        let result = parse_decks(CARDS);
        assert!(result.is_ok());

        let (deck0, deck1) = result.unwrap();
        assert_eq!(vec![9, 2, 6, 3, 1], deck0.cards);
        assert_eq!(vec![5, 8, 4, 7, 10], deck1.cards);
    }

    #[test]
    fn test_play_round() {
        let (mut player1, mut player2) = parse_decks(CARDS).unwrap();

        assert_eq!(0, play_round(&mut player1, &mut player2));
        assert_eq!(vec![2, 6, 3, 1, 9, 5], player1.cards);
        assert_eq!(vec![8, 4, 7, 10], player2.cards);
    }

    #[test]
    fn test_play_game_1() {
        let (player1, player2) = parse_decks(CARDS).unwrap();
        let winner = play_game_1(player1, player2);

        assert_eq!(1, winner.player_id);
        assert_eq!(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1], winner.cards);
    }

    #[test]
    fn test_calculate_score() {
        let (player1, player2) = parse_decks(CARDS).unwrap();
        let winner = play_game_1(player1, player2);

        assert_eq!(306, winner.score());
    }

    #[test]
    fn test_player_game_recursive() {
        let (player1, player2) = parse_decks(CARDS).unwrap();
        let mut game = GameRecursive::new(1, player1, player2);

        let winner = game.play();
        assert_eq!(1, winner.player_id);
        assert_eq!(vec![7, 5, 6, 2, 4, 1, 10, 8, 9, 3], winner.cards);
        assert_eq!(291, winner.score());
    }
}
