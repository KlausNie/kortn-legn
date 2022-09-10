use crate::{CardNumber};
use crate::deck::card_color::CardColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub color: CardColor,
    pub number: CardNumber,
}