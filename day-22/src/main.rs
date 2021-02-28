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
}

/// Play a single round
fn play_round(player1: &mut Deck, player2: &mut Deck) {

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

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::parse_decks;

    const CARDS: &str = r#"
        Player 1:
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
}
