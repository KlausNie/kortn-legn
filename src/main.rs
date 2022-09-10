use crate::deck::{create_deck, Ordered, randomize};
use crate::deck::def::{Card, CardColor, CardNumber};

mod deck;
mod game;

fn main() {

    let deck: Vec<Card> = create_deck();
    let field = game::initField(randomize(deck));
//     println!("{:?}", shuffled);
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

    println!("in order: {:?}", field.not_played_cards.is_in_order());


    // println!("{:?}", field);
}