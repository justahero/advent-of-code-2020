use anyhow::anyhow;

#[derive(Debug, Clone)]
struct Deck {
    pub name: String,
    pub cards: Vec<u64>,
}

impl Deck {
    /// Parses the name and list of cards
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        // parse player name
        let lines = content
            .lines()
            .map(str::trim)
            .collect::<Vec<_>>();

        let name = lines.first().ok_or_else(|| anyhow!("Name not found"))?;
        let cards = lines[1..]
            .iter()
            .map(|&v| v.parse::<u64>())
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        Ok(Self {
            name: name.to_string(),
            cards,
        })
    }

    /// Show the top card of the deck
    pub fn top_card(&self) -> Option<&u64> {
        self.cards.first()
    }

    /// Returns true if deck has no cards
    pub fn empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Draw top card from other player, put on to bottom of card deck
    pub fn put_cards(&mut self, other: &mut Deck)  {
        let left_card = self.cards.remove(0);
        let right_card = other.cards.remove(0);
        self.cards.append(&mut vec![left_card, right_card]);
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
}

/// Plays the game between players
fn play_game(mut player1: Deck, mut player2: Deck) -> Deck {
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

/// Play a single round, transfers cards accordingly, returns player who won
fn play_round(player1: &mut Deck, player2: &mut Deck) -> u64 {
    if player1.top_card() > player2.top_card() {
        player1.put_cards(player2);
        0
    } else {
        player2.put_cards(player1);
        1
    }
}

/// Parses the text content into two decks of cards
fn parse_decks(content: &str) -> anyhow::Result<(Deck, Deck)> {
    let parts = content
        .split("\n\n");

    let decks = parts
        .map(|item| Deck::parse(item))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok((decks[0].clone(), decks[1].clone()))
}

fn main() -> anyhow::Result<()> {
    let (player1, player2) = parse_decks(include_str!("cards.txt"))?;
    let winner = play_game(player1, player2);

    dbg!(winner.score());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_decks, play_game, play_round};

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
    fn test_play_game() {
        let (player1, player2) = parse_decks(CARDS).unwrap();
        let winner = play_game(player1, player2);

        assert_eq!("Player 2:".to_string(), winner.name);
        assert_eq!(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1], winner.cards);
    }

    #[test]
    fn test_calculate_score() {
        let (player1, player2) = parse_decks(CARDS).unwrap();
        let winner = play_game(player1, player2);

        assert_eq!(306, winner.score());
    }
}
