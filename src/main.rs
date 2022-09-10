mod deck;
mod game;
mod ordered;
mod randomize;

use crate::deck::{create_deck};
use crate::deck::def::{Card, CardColor, CardNumber};
use crate::ordered::Ordered;
use crate::randomize::Randomize;


fn main() {
    let deck: Vec<Card> = create_deck();
    let shuffled = deck.randomize();
    println!("{:?}", shuffled.clone());
    let field = game::init_field(shuffled);
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