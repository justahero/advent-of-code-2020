# Day 22

[Advent Of Code Day 22](https://adventofcode.com/2020/day/22).

## Part 1

Done

## Part 2

Card game, two decks, two players, rules are Recursive Combat

* game consists of a series of rounds
* before either player deals a card, if there was a previous round in this game, that had exactly the same cards in the same order in the same players' decks, the game instantly ends in a win for player 1
* previous rounds from other games (?) are not considered
* otherwise the round's cards must be in a new configuration
  * players begin the round by each drawing top card
  * if **both** players have at least as many cards remaining in their deck as the value of the card they just drew, the winner of the round is determined by playing a new game
  * otherwise, at least one player may not have enough cards left in their deck to recurse, the winner of the round is the player with the higher value card.
