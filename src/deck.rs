pub mod def;

use crate::deck::def::{Card, CardNumber, CardColor};

const COLORS_ARRAY: [CardColor; 4] = [
    CardColor::Herz,
    CardColor::Laab,
    CardColor::Oachl,
    CardColor::Schell
];
const NUMBERS_ARRAY: [CardNumber; 8] = [
    CardNumber::Siebma,
    CardNumber::Ochta,
    CardNumber::Neina,
    CardNumber::Zehna,
    CardNumber::Unto,
    CardNumber::Obo,
    CardNumber::Kinig,
    CardNumber::Ass,
];

pub fn create_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(32);
    for card_color in COLORS_ARRAY {
        for card_number in NUMBERS_ARRAY {
            cards.push(
                Card {
                    color: card_color,
                    number: card_number,
                }
            );
        }
    }

    return cards;
}

pub fn randomize(cards: Vec<Card>) -> Vec<Card> {
    return cards;
}