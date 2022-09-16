use crate::deck::card::Card;
use crate::deck::create_deck::create_deck;
use crate::deck::randomize::Randomize;
use crate::field::field::Field;
use crate::field::init_field::init_field;

pub(crate) fn setup() -> Field {
    let deck: Vec<Card> = create_deck();
    let shuffled = deck.randomize();
    init_field(shuffled)
}