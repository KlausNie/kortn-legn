extern crate core;

mod deck;
mod field;

use crate::deck::card::Card;
use crate::deck::card_color::CardColor;
use crate::deck::card_number::CardNumber;
use crate::deck::create_deck::create_deck;
use crate::field::init_field::init_field;
use deck::randomize::Randomize;
use crate::field::field::Field;
use crate::field::play::{Play, PlaySource, PlayTarget};


fn main() {
    // check_cards();

    let mut field = setup();
    println!("{:?}", field.not_played_cards.len());

    // while field.not_played_cards.len() > 0 {
    field = field.play_card(PlaySource::BottomStack1, PlayTarget::TopStack1);
    println!("bottom_stack1: {:?}", field.bottom_stack1.len());
    println!("top_stack1: {:?}", field.top_stack1.len());

    // }

    // println!("in order: {:?}", field.not_played_cards.is_in_order());
}

fn setup() -> Field {
    let deck: Vec<Card> = create_deck();
    let shuffled = deck.randomize();
    // println!("{:?}", shuffled.clone());
    let field = init_field(shuffled);
    field
}

fn check_cards() {
    let c1 = Card {
        color: CardColor::Herz,
        number: CardNumber::Ass
    };
    let c2 = Card {
        color: CardColor::Schell,
        number: CardNumber::Siebma
    };
    println!("{:?}", c1);
    println!("{:?}", c1.cmp(&c2));
    println!("{:?}", c2);
    println!("{:?}", c2);
    println!("{:?}", c2.cmp(&c1));
    println!("{:?}", c1);
}