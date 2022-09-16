use game::deck::card::Card;
use game::deck::create_deck::create_deck;
use game::deck::randomize::Randomize;
use game::field::field::Field;
use game::field::init_field::init_field;

pub(crate) fn setup() -> Field {
    let deck: Vec<Card> = create_deck();
    let shuffled = deck.randomize();
    init_field(shuffled)
}