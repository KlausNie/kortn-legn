extern crate core;

mod deck;
mod field;

use std::{time};
use rand::{Rng, thread_rng};
use crate::deck::card::Card;
use crate::deck::card_color::CardColor;
use crate::deck::card_number::CardNumber;
use crate::deck::create_deck::create_deck;
use crate::field::init_field::init_field;
use deck::randomize::Randomize;
use crate::deck::ordered::Ordered;
use crate::field::field::Field;
use crate::field::play::{Play, PlaySource, PlayTarget};


fn main() {
    // check_cards();

    let mut field = setup();
    // println!("{:?}", field.not_played_cards.len());

    // field = field.play_card(&PlaySource::BottomStack1, &PlayTarget::TopStack1);
    // println!("bottom_stack1: {:?}", field.bottom_stack1.len());
    // println!("top_stack1: {:?}", field.top_stack1.len());

    let sources: [PlaySource; 4] = [
        PlaySource::NotPlayedCards,
        PlaySource::BottomStack1,
        PlaySource::BottomStack2,
        PlaySource::BottomStack3,
    ];
    let target: [PlayTarget; 5] = [
        PlayTarget::BottomStack1,
        PlayTarget::BottomStack2,
        PlayTarget::BottomStack3,
        PlayTarget::TopStack1,
        PlayTarget::TopStack2,
    ];

    while field.not_played_cards.len() > 0 {
        // let ten_millis = time::Duration::from_millis(100);
        // thread::sleep(ten_millis);
        // let source = &PlaySource::NotPlayedCards; // sources[random_source(sources.len())].borrow();
        // let target = &PlayTarget::TopStack1; // target[random_source(target.len())].borrow();
        let source = sources[random_source(sources.len())];
        let target = target[random_source(target.len())];

        let can_play = field.can_play(source, target);
        // println!("can play {:?} {:?} {:?}", source, target, can_play);
        if can_play {
            println!("playing from {:?} {:?}", source, target);
            field = field.play_card(source, target);
        }
    }

    println!("top_stack1: {:?}", field.top_stack1);
    println!("top_stack2: {:?}", field.top_stack2);
    println!("top_stack1 in order: {:?}", field.top_stack1.is_in_order());
    println!("top_stack2 in order: {:?}", field.top_stack2.is_in_order());
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

fn random_source(max: usize) -> usize {
    let mut rng = thread_rng();
    return rng.gen_range(0..max);
}