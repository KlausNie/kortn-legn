extern crate core;

mod deck;
mod field;
mod play;

use crate::deck::card::Card;
use crate::deck::card_color::CardColor;
use crate::deck::card_number::CardNumber;
use crate::deck::ordered::Ordered;
use crate::field::field::Field;
use crate::field::card_movement::{PlaySource, PlayTarget};
use crate::field::playable::{OverallGameState, Playable};
use crate::play::{play};

fn main() {
    let mut c = 0;
    while c < 1000 {
        let result_field = play();
        if result_field.finished() == OverallGameState::Success {
            println!("Game {:?} ended with result {:?} ({})", c, result_field.finished(), result_field.stats());
            println!("not_played_cards: {:?}", result_field.not_played_cards);
            println!("top_stack1: {:?} {:?}", result_field.top_stack1.is_in_order(), result_field.top_stack1);
            println!("top_stack2: {:?} {:?}", result_field.top_stack2.is_in_order(), result_field.top_stack2);
            println!("bottom_stack1: {:?}", result_field.bottom_stack1);
            println!("bottom_stack2: {:?}", result_field.bottom_stack2);
            println!("bottom_stack3: {:?}", result_field.bottom_stack3);
        } else {
            println!("Game {:?} ended with result {:?} ({})", c, result_field.finished(), result_field.stats())
        }
        c += 1;
    }
}
