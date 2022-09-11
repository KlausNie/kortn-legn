use std::fmt;
use crate::{CardNumber};
use crate::deck::card_color::CardColor;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub color: CardColor,
    pub number: CardNumber,
}

/// more dense implementation of debug print
impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("num", &self.number)
            // .field("col", &self.color)
            .finish()
    }
}
