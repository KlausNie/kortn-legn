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

    pub(crate) fn is_higher_or_less_than(&self, other: &Card, distance: u16) -> bool {
        if self.is_next_higher(other) {
            return true
        }
        self.number.is_somewhat_higher(&other.number, distance)
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

    #[test]
    fn is_higher_or_less_than() {
        let herz_ass = Card {
            color: CardColor::Herz,
            number: CardNumber::Ass
        };
        let herz_siebma = Card {
            color: CardColor::Herz,
            number: CardNumber::Siebma
        };
        let herz_ochta = Card {
            color: CardColor::Herz,
            number: CardNumber::Ochta
        };
        let herz_neina = Card {
            color: CardColor::Herz,
            number: CardNumber::Neina
        };
        let herz_zehna = Card {
            color: CardColor::Herz,
            number: CardNumber::Zehna
        };
        let herz_unto = Card {
            color: CardColor::Herz,
            number: CardNumber::Unto
        };

        assert!(herz_siebma.is_higher_or_less_than(&herz_ass, 3));
        assert!(herz_ochta.is_higher_or_less_than(&herz_ass, 3));
        assert!(herz_neina.is_higher_or_less_than(&herz_ass, 3));
        assert!(!herz_zehna.is_higher_or_less_than(&herz_ass, 3));
        assert!(!herz_unto.is_higher_or_less_than(&herz_ass, 3));
    }
}