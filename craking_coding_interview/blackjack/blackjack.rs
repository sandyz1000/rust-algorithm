/* 
## Cloning trait object 
https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5


trait Foo {
    fn box_clone(&self) -> Box<Foo>;
}

impl Clone for Box<Foo>
{
    fn clone(&self) -> Box<Foo> {
        self.box_clone()
    }
}

#[derive(Clone)]
struct Bar;

impl Foo for Bar {
    fn box_clone(&self) -> Box<Foo> {
        Box::new((*self).clone())
    }
}

#[test]
fn it_works() {
    let baz = Box::new(Bar) as Box<Foo>;
    let qux = baz.clone();
}


*/
#![allow(unused)]

use std::vec::Vec;
use chrono::Utc;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Suit {
    Heart = 1,
    Spade = 2,
    Club = 3,
    Diamond = 4,
}

impl Default for Suit {
    fn default() -> Self {
        Suit::Heart
    }
}


trait Card {
    fn get_suit(&self) -> &Suit;

    fn get_face_value(&self) -> i32;
}


#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct BlackjackCard {
    suit: Suit,
    face_value: i32,
    game_value: i32,
}

impl BlackjackCard {
    fn new(suit: Suit, face_value: i32) -> Self {
        let game_value = if face_value > 10 { 10 } else { face_value };
        Self {
            suit,
            face_value,
            game_value,
        }
    }
}

impl Card for BlackjackCard { 
    fn get_suit(&self) -> &Suit {
        &self.suit
    }

    fn get_face_value(&self) -> i32 {
        self.face_value
    }
}

struct Deck {
    cards: Vec<BlackjackCard>,
    creation_date: chrono::NaiveDateTime,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();
        let creation_date = Utc::now().naive_utc();
        for value in 1..=13 {
            for suit in &[Suit::Heart, Suit::Spade, Suit::Club, Suit::Diamond] {
                cards.push(BlackjackCard::new(suit.clone(), value));
            }
        }
        Deck {
            cards,
            creation_date,
        }
    }

    fn get_cards(&self) -> &Vec<BlackjackCard> {
        &self.cards
    }
}

struct Shoe {
    cards: Vec<Vec<BlackjackCard>>,
    number_of_decks: usize,
}

impl Shoe {
    fn new(number_of_decks: usize) -> Self {
        let mut cards: Vec<Vec<BlackjackCard>> = Vec::new();
        for _ in 0..number_of_decks {
            let deck = Deck::new();
            let deck_cards: Vec<BlackjackCard> = deck.get_cards().clone();
            cards.push(deck_cards);
        }
        Shoe {
            cards,
            number_of_decks,
        }
    }

    fn create_shoe(&mut self) {
        self.cards.clear();
        for _ in 0..self.number_of_decks {
            let deck = Deck::new();
            self.cards.push(deck.get_cards().clone());
        }
    }

    fn shuffle(&mut self) {
        let card_count = self.cards.iter().map(|deck| deck.len()).sum();
        let mut rng = thread_rng();
        for i in 0..card_count {
            let j = rng.gen_range(i..card_count);
            let (deck_i, card_i) = self.get_deck_and_card_index(i);
            let (deck_j, card_j) = self.get_deck_and_card_index(j);
            self.cards[deck_i][card_i] = std::mem::replace(&mut self.cards[deck_j][card_j], Default::default());
        }
    }

    fn get_deck_and_card_index(&self, index: usize) -> (usize, usize) {
        let mut deck_index = 0;
        let mut card_index = index;
        while card_index >= self.cards[deck_index].len() {
            card_index -= self.cards[deck_index].len();
            deck_index += 1;
        }
        (deck_index, card_index)
    }

    fn deal_card(&mut self) -> BlackjackCard {
        if self.cards.is_empty() || self.cards.iter().all(|deck| deck.is_empty()) {
            self.create_shoe();
            self.shuffle();
        }
        let (deck_index, card_index) = self.get_deck_and_card_index(0);
        self.cards[deck_index].remove(card_index)
    }
}

struct Person;

trait BasePlayer {
    fn get_hands(&self) -> &Vec<Hand>;
    fn add_hand(&mut self, hand: Hand);
    fn remove_hand(&mut self, hand: &Hand);
    fn get_total_score(&self) -> i32 {
        0
    }
    
    fn reset_password(&self) {
        // Implementation for resetting password
    }

    fn place_bet(&mut self, bet: f32) {
        
    }


}

struct Player {
    id: String,
    password: String,
    balance: f32,
    status: String,
    person: Person,
    hands: Vec<Hand>,
    bet: f32,
    total_cash: f32,
}

impl Player {
    fn new(id: String, password: String, balance: f32, status: String, person: Person) -> Self {
        Self {
            id,
            password,
            balance,
            status,
            person,
            hands: Vec::new(),
            bet: 0.0,
            total_cash: 0.0
        }
    }
}

impl BasePlayer for Player {
    fn get_hands(&self) -> &Vec<Hand> {
        &self.hands
    }

    fn add_hand(&mut self, hand: Hand) {
        self.hands.push(hand);
    }

    fn remove_hand(&mut self, hand: &Hand) {
        if let Some(index) = self.hands.iter().position(|h| *h == *hand) {
            self.hands.remove(index);
        }
    }
}

struct Dealer {
    id: String,
    password: String,
    balance: f32,
    status: String,
    person: Person,
    hands: Vec<Hand>,
}

impl Dealer {
    fn new(id: String, password: String, balance: f32, status: String, person: Person) -> Self {
        Self {
            id,
            password,
            balance,
            status,
            person,
            hands: Vec::new(),
        }
    }
}

impl BasePlayer for Dealer { 

    fn get_hands(&self) -> &Vec<Hand> {
        &self.hands
    }

    fn add_hand(&mut self, hand: Hand) {
        self.hands.push(hand);
    }

    fn remove_hand(&mut self, hand: &Hand) {
        if let Some(index) = self.hands.iter().position(|h| h == hand) {
            self.hands.remove(index);
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<BlackjackCard>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {

}

impl Hand {
    fn new(blackjack_card1: BlackjackCard, blackjack_card2: BlackjackCard) -> Self {
        Hand {
            cards: vec![blackjack_card1, blackjack_card2],
        }
    }

    fn get_scores(&self) -> Vec<i32> {
        let mut totals = vec![0];

        for card in &self.cards {
            let mut new_totals = Vec::new();
            for score in &totals {
                new_totals.push(card.get_face_value() + score);
                if card.get_face_value() == 1 {
                    new_totals.push(11 + score);
                }
            }
            totals = new_totals;
        }

        totals
    }

    fn get_cards(&self) -> &Vec<BlackjackCard> {
        &self.cards
    }

    fn add_card(&mut self, card: BlackjackCard) {
        self.cards.push(card);
    }

    fn resolve_score(&self) -> i32 {
        let scores = self.get_scores();
        let mut best_score = 0;
        for score in scores {
            if score <= 21 && score > best_score {
                best_score = score;
            }
        }
        best_score
    }
}


fn get_bet_from_ui() -> f32 {
    // Implementation for getting bet from UI
    0.0
}

fn get_user_action(hand: &Hand) -> String {
    // Implementation for getting user action
    String::new()
}

struct Game {
    player: Player,
    dealer: Dealer,
    max_num_of_decks: usize,
    shoe: Shoe,
}

impl Game {
    fn new(player: Player, dealer: Dealer) -> Self {
        Game {
            player,
            dealer,
            max_num_of_decks: 3,
            shoe: Shoe::new(3),
        }
    }

    fn play_action(&mut self, action: &str, hand: &mut Hand) {
        match action {
            "hit" => self.hit(hand),
            "split" => self.split(hand),
            "stand pat" => {} // do nothing
            "stand" => self.stand(),
            _ => println!("Invalid move"),
        }
    }

    fn hit(&mut self, hand: &mut Hand) {
        hand.add_card(self.shoe.deal_card());
    }

    fn stand(&mut self) {
        let dealer_score = self.dealer.get_total_score();
        let player_score = self.player.get_total_score();
        let hands = self.player.get_hands();
        for hand in hands {
            let best_score = hand.resolve_score();
            if player_score == 21 {
                // blackjack, pay 3:2 of the bet
            } else if player_score > dealer_score {
                // pay player equal to the bet
            } else if player_score < dealer_score {
                // collect the bet from the player
            } else {
                // tie, bet goes back to player
            }
        }
    }

    fn split(&mut self, hand: &mut Hand) {
        let cards = hand.get_cards().clone();
        self.player.add_hand(Hand::new(cards[0].clone(), self.shoe.deal_card()));
        self.player.add_hand(Hand::new(cards[1].clone(), self.shoe.deal_card()));
        self.player.remove_hand(hand);
    }

    fn start(&mut self) {
        self.player.place_bet(get_bet_from_ui());
        self.player.add_hand(Hand::new(self.shoe.deal_card(), self.shoe.deal_card()));

        self.dealer.add_hand(Hand::new(self.shoe.deal_card(), self.shoe.deal_card()));

        loop {
            let hands = self.player.get_hands().clone();
            for mut hand in hands {
                let action = get_user_action(&hand);
                self.play_action(&action, &mut hand);
                if action == "stand" {
                    break;
                }
            }
        }
    }
}

fn main() {
    let player = Player::new("".to_string(), "".to_string(), 0.0, "".to_string(), Person);
    let dealer = Dealer::new("".to_string(), "".to_string(), 0.0, "".to_string(), Person);
    let mut game = Game::new(player, dealer);
    game.start();
}
