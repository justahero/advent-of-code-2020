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

        println!("DECK: {:?}", lines);

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

    /// Draw top card from other player, put on to bottom of card deck
    pub fn put_cards(&mut self, other: &mut Deck)  {
        let left_card = self.cards.remove(0);
        let right_card = other.cards.remove(0);
        self.cards.append(&mut vec![left_card, right_card]);
    }
}

/// Play a single round, transfers cards accordingly, returns player who won
fn play_round(player1: &mut Deck, player2: &mut Deck) -> u64 {
    println!("Round");
    println!("Player 1 {}", player1.top_card().unwrap());
    println!("Player 2 {}", player2.top_card().unwrap());

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

    dbg!(&parts);

    let decks = parts
        .map(|item| Deck::parse(item))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    dbg!(&decks);

    Ok((decks[0].clone(), decks[1].clone()))
}

fn main() -> anyhow::Result<()> {
    let (mut player1, mut player2) = parse_decks(include_str!("cards.txt"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_decks, play_round};

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
}
