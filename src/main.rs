extern crate core;

mod deck;
mod field;

use std::{thread, time};
use rand::{Rng, thread_rng};
use crate::deck::card::Card;
use crate::deck::card_color::CardColor;
use crate::deck::card_number::CardNumber;
use crate::deck::create_deck::create_deck;
use crate::field::init_field::init_field;
use deck::randomize::Randomize;
use crate::deck::ordered::Ordered;
use crate::field::best_play::HasBestPlay;
use crate::field::field::Field;
use crate::field::play::{Play, PlaySource, PlayTarget};
use crate::field::playable::Playable;

fn main() {
    let mut field = setup();

    let sources: [PlaySource; 4] = [
        PlaySource::NotPlayedCards,
        PlaySource::BottomStack1,
        PlaySource::BottomStack2,
        PlaySource::BottomStack3,
    ];
    let targets: [PlayTarget; 5] = [
        PlayTarget::BottomStack1,
        PlayTarget::BottomStack2,
        PlayTarget::BottomStack3,
        PlayTarget::TopStack1,
        PlayTarget::TopStack2,
    ];

    while !field.finished() {
        // let ten_millis = time::Duration::from_millis(100);
        // thread::sleep(ten_millis);
        // let source = &PlaySource::NotPlayedCards; // sources[random_source(sources.len())].borrow();
        // let target = &PlayTarget::TopStack1; // target[random_source(target.len())].borrow();

        match field.best_play() {
            HasBestPlay::BestPlay(source, target) => {
                let can_play = field.can_play(source, target);
                // println!(
                //     "best play: {:?} {:?} {:?} {:?} {:?}",
                //     source,
                //     target,
                //     can_play,
                //     field.get_source(source).last(),
                //     field.get_target(target).last()
                // );
                if can_play {
                    // println!("playing best play from {:?} to {:?}", source, target);
                    field = field.play_card(source, target);
                }
            }
            HasBestPlay::None => {
                let source = sources[random_nr_0_to(sources.len())];
                let target = targets[random_nr_0_to(targets.len())];

                let can_play = field.can_play(source, target);
                // println!("random suggestion: {:?} {:?} {:?}", source, target, can_play);
                if can_play {
                    // println!("random play {:?} to {:?}", source, target);
                    field = field.play_card(source, target);
                }
            }
        }
    }

    println!("not_played_cards: {:?}", field.not_played_cards);
    println!("top_stack1: {:?} {:?}", field.top_stack1.is_in_order(), field.top_stack1);
    println!("top_stack2: {:?} {:?}", field.top_stack2.is_in_order(), field.top_stack2);
    println!("bottom_stack1: {:?}", field.bottom_stack1);
    println!("bottom_stack2: {:?}", field.bottom_stack2);
    println!("bottom_stack3: {:?}", field.bottom_stack3);
}

fn setup() -> Field {
    let deck: Vec<Card> = create_deck();
    let shuffled = deck.randomize();
    init_field(shuffled)
}

fn random_nr_0_to(max: usize) -> usize {
    let mut rng = thread_rng();
    return rng.gen_range(0..max);
}