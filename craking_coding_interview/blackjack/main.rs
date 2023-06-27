// Deck of Cards:
// --------------
//
// Problem definition
// Blackjack is one of the most played games in casinos. In this game, several players play against the dealer.
// The objective of the game is to get closer to 21 than the dealer without exceeding the 21 points. This game
// is played with a deck of cards. Each card has a specific value associated with it, and these values are compared.
// The value of an ace card can be 1 or 11 points while cards of 10, Jack, Queen, and King value 10 points.
// Whereas, cards 2-9 have the same values as the number written on them. Hence, the player having one ace and a
// face card (King, Queen, or Jack card.) or a 10 card has 21 points. This is called Blackjack.
//
// At the start of the game, a bet is placed by a player, and then the dealer will create hands (Each player and
// the dealer get two cards each.). All players have both cards exposed while the dealer has one card exposed
// and one card hidden. Now, both the dealer and the player can hit (Draw an additional card.) a card and ensure
// that they should not get over 21. Anyone exceeding 21 points, busts and loses the bet. The player can stand pat
// (Stop taking cards) at any time. When a player stops taking cards, if the player has more points than the dealer
// but less than 22, they win the bet.
//
//
// Design the data structures for a generic deck of cards.
// Explain how you would subclass the data structures to implement blackjack.
// https://bicyclecards.com/how-to-play/blackjack/
// https://www.educative.io/courses/grokking-the-low-level-design-interview-using-ood-principles/B8E5kYVjonW
//
#![allow(unused)]
use std::io;
use std::{thread, time};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", &self.rank, &self.suit)
    }
}

#[derive(EnumIter, Debug, PartialEq, Clone)]
#[allow(unused_variables, dead_code)]
pub enum Suit {
    Hearts,
    Tiles,
    Clovers,
    Pikes,
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
#[allow(unused_variables, dead_code)]
pub enum Rank {
    // Todo: Refactor so Two = 2, etc and in game use 'Two as u8' instead of that match statement.
    Ace,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Knight,
    Queen,
    King,
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn add_card(&mut self, c: Card) {
        self.cards.push(c);
    }
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit.clone(), rank.clone()))
            }
        }
        cards.push(Card::new(Suit::Clovers, Rank::Ace));
        Self { cards }
    }
    pub fn get_card(&mut self) -> Card {
        let card = self.cards.pop();
        match card {
            None => panic!("Not enough cards in deck."),
            Some(card) => card,
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        &self.cards.shuffle(&mut rng);
    }
}

pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}

pub struct Dealer {
    pub name: String,
    hand: Vec<Card>,
}

pub trait Person {
    //  fn new(name: &str) -> Self;
    fn deal_card(&mut self, card: Card);
    fn get_hand(&self) -> &Vec<Card>;
    fn next_move(&self) -> Reply;
    fn get_name(&self) -> &str;
}
impl Dealer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            hand: Vec::<Card>::new(),
        }
    }
}

impl Person for Dealer {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    fn next_move(&self) -> Reply {
        Reply::AskUI
    }
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            hand: Vec::<Card>::new(),
        }
    }
}

impl Person for Player {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn deal_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    fn next_move(&self) -> Reply {
        Reply::AskUI
    }
}

pub enum Reply {
    Hit,
    Stand,
    Split,
    AskUI,
}

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Whoops something went wrong");
    print!("{}[2J", 27 as char);
    input
}

pub fn display_playerhand(player: &str, hand: &Vec<Card>, score: Score) {
    println!("{} \n=============", player);
    for card in hand.iter() {
        println!("{}", card)
    }
    println!("Score: {}\n", score)
}

pub fn player_wants_to_hit() -> bool {
    let res = get_input("Do you want to hit? \n 'y' for yes and 'n' for no.");

    if res.contains("y") {
        return true;
    } else if res.contains("n") {
        return false;
    } else {
        println!("What? Try again.\n");
        player_wants_to_hit()
    }
}

pub fn announce_winner(name: &str, score: Score) {
    println!("The winner is....");
    thread::sleep(time::Duration::from_millis(1500));
    println!("{} with a score of {}!\n", name, score);
}

pub fn play_again() -> bool {
    let res = get_input("Play again?\n 'y' for yes and 'n' for no.");
    if res.contains("y") {
        return true;
    } else if res.contains("n") {
        return false;
    } else {
        println!("What? Try again.\n");
        play_again()
    }
}

pub fn announce_dealing(card: &Card, name: &str) {
    print!("{}[2J", 27 as char);
    println!("Dealing card...");
    thread::sleep(time::Duration::from_millis(1500));
    print!("{}[2J", 27 as char);
    println!("Dealing card {} to {}.\n\n", card, name);
    thread::sleep(time::Duration::from_millis(1500));
    print!("{}[2J", 27 as char);
}

pub fn deal_players(deck: &mut Deck, dealer: &mut dyn Person, player: &mut dyn Person) {
    player.deal_card(deck.get_card());
    dealer.deal_card(deck.get_card());
    player.deal_card(deck.get_card());
}

pub fn get_score(player: &dyn Person) -> Score {
    let hand = &player.get_hand();
    let mut result: u8 = 0;
    for card in hand.iter() {
        match card.rank {
            Rank::Knight => result += 10,
            Rank::Queen => result += 10,
            Rank::King => result += 10,
            Rank::Ace => result += 1,
            x => result += x as u8,
        }
    }
    for card in hand.iter() {
        if let Rank::Ace = card.rank {
            if result + 10 <= 21 {
                result += 10;
            }
        }
    }
    if result > 21 {
        return Score::Busted;
    }
    if result == 21 && hand.len() == 2 {
        return Score::Blackjack;
    }
    Score::Points(result)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Score {
    Busted,
    Points(u8),
    Blackjack,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Score::Blackjack => write!(f, "{:?}", &self),
            Score::Busted => write!(f, "{:?}", &self),
            Score::Points(num) => write!(f, "{}", num),
        }
    }
}

pub fn get_winner<'a>(
    dealer: &'a dyn Person, player: &'a dyn Person
) -> Option<&'a dyn Person> {
    let player_score = get_score(player); // Get their scores.
    let dealer_score = get_score(dealer);
    if player_score == dealer_score {
        // If they have the same score but not blackjack
        if dealer_score > Score::Points(16) && dealer_score != Score::Blackjack {
            return Some(dealer); // The dealer wins from 17 and up
        } else {
            return None;
        }
    };
    if player_score > dealer_score {
        // Otherwise player with the highest score wins.
        return Some(player);
    } else {
        return Some(dealer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queen_and_ace_is_blackjack() {
        let mut player = Player::new("Test");
        player.deal_card(Card::new(Suit::Clovers, Rank::Ace));
        player.deal_card(Card::new(Suit::Clovers, Rank::Queen));
        assert_eq!(Score::Blackjack, get_score(&mut player));
    }

    #[test]
    fn seven_and_ace_is18() {
        let mut player = Player::new("Test");
        player.deal_card(Card::new(Suit::Clovers, Rank::Seven));
        player.deal_card(Card::new(Suit::Hearts, Rank::Ace));
        assert_eq!(Score::Points(18), get_score(&mut player));
    }

    #[test]
    fn seven_ace_and_knight_is18() {
        let mut player = Player::new("Test");
        player.deal_card(Card::new(Suit::Clovers, Rank::Seven));
        player.deal_card(Card::new(Suit::Hearts, Rank::Ace));
        player.deal_card(Card::new(Suit::Tiles, Rank::Knight));
        assert_eq!(Score::Points(18), get_score(&mut player));
    }

    #[test]
    fn five_aces_is15() {
        let mut player = Player::new("Test");
        for _num in 0..5 {
            player.deal_card(Card::new(Suit::Hearts, Rank::Ace));
        }
        assert_eq!(Score::Points(15), get_score(&mut player));
    }

    #[test]
    fn blackjack_wins_over_21() {
        let mut player = Player::new("Test");
        let mut dealer = Player::new("Test2");
        player.deal_card(Card::new(Suit::Hearts, Rank::Ace));
        player.deal_card(Card::new(Suit::Hearts, Rank::Ten));

        dealer.deal_card(Card::new(Suit::Hearts, Rank::Ten));
        dealer.deal_card(Card::new(Suit::Hearts, Rank::Ten));
        dealer.deal_card(Card::new(Suit::Hearts, Rank::Ace));
        let winner = get_winner(&dealer, &player).unwrap();
        assert_eq!(player.get_name(), winner.get_name());
    }

    #[test]
    fn aces_shrink_when_needed() {
        let mut player = Player::new("Player");
        player.deal_card(Card::new(Suit::Hearts, Rank::Ten));
        player.deal_card(Card::new(Suit::Clovers, Rank::Ace));
        player.deal_card(Card::new(Suit::Tiles, Rank::Ace));
        let score = get_score(&player);
        assert_eq!(Score::Points(12), score);
    }
}

fn main() {
    loop {
        let mut deck = Deck::new(); // Create a new deck
        deck.shuffle(); // Shuffle it.

        // Create some players.
        let mut player = Player::new("Player 1");
        let mut dealer = Dealer::new("Dealer");

        deal_players(&mut deck, &mut dealer, &mut player); // Give the players their initial cards.

        // Display inital hand
        display_playerhand(
            &dealer.name, dealer.get_hand(), get_score(&dealer)
        );
        display_playerhand(
            &player.name, player.get_hand(), get_score(&player)
        );

        // The player chooses if they want more cards.
        while get_score(&player) < Score::Points(22) && player_wants_to_hit() {
            let card = deck.get_card();

            announce_dealing(&card, &player.name);
            player.deal_card(card);
            if get_score(&player) == Score::Busted {
                display_playerhand(
                    &dealer.name, 
                    dealer.get_hand(), 
                    get_score(&dealer)
                );
                display_playerhand(
                    &player.name, 
                    player.get_hand(), 
                    get_score(&player)
                );
                break;
            } 
            else if get_score(&player) == Score::Blackjack {
                display_playerhand(
                    &dealer.name, 
                    dealer.get_hand(), 
                    get_score(&dealer)
                );
                display_playerhand(
                    &player.name, 
                    player.get_hand(), 
                    get_score(&player)
                );
                break;
            } else {
                display_playerhand(
                    &dealer.name, 
                    dealer.get_hand(), 
                    get_score(&dealer)
                );
                display_playerhand(
                    &player.name, 
                    player.get_hand(), 
                    get_score(&player)
                );
            }
        }
        
        // Dealer's turn.
        if get_score(&player) != Score::Busted && get_score(&player) != Score::Blackjack {
            let mut dealer_score = get_score(&dealer);
            while dealer_score < Score::Points(17) && dealer_score != Score::Busted {
                let card = deck.get_card();
                announce_dealing(&card, &dealer.name);
                dealer.deal_card(card);
                dealer_score = get_score(&dealer);
            }
            display_playerhand(
                &dealer.name, 
                dealer.get_hand(), 
                get_score(&dealer)
            );
            display_playerhand(
                &player.name, 
                player.get_hand(), 
                get_score(&player)
            );
        }

        let winner = get_winner(&dealer, &player);
        match winner {
            Some(winner) => announce_winner(&winner.get_name(), get_score(winner)),
            None => println!("It's a tie!"),
        }

        if !play_again() {
            break;
        }
    }
}
