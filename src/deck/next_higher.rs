use CardNumber::Ochta;
use crate::deck::card::Card;
use crate::deck::card_number::CardNumber;
use crate::deck::card_number::CardNumber::{Ass, Kinig, Neina, Obo, Siebma, Unto, Zehna};

/// shitty implementation to check if this card is the next higher one to the other
/// doesn't even respect the color
impl Card {
    pub(crate) fn is_next_higher(&self, other: &Card) -> bool {
        if self.number == Siebma && other.number == Ass {
            return true
        }
        return match self.number {
            Siebma => {
                other.number == Ass
            }
            Ochta => {
                other.number == Siebma
            }
            Neina => {
                other.number == Ochta
            }
            Zehna => {
                other.number == Neina
            }
            Unto => {
                other.number == Zehna
            }
            Obo => {
                other.number == Unto
            }
            Kinig => {
                other.number == Obo
            }
            Ass => {
                other.number == Kinig
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::card_color::CardColor;
    use crate::deck::card_number::CardNumber;
    use super::{Card};

    #[test]
    fn is_next_higher() {
        let herz_ass = Card {
            color: CardColor::Herz,
            number: CardNumber::Ass
        };
        let herz_siebma = Card {
            color: CardColor::Herz,
            number: CardNumber::Siebma
        };

        assert!(herz_siebma.is_next_higher(&herz_ass));
        assert!(!herz_ass.is_next_higher(&herz_siebma));
    }
}