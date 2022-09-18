use crate::deck::card_number::CardNumber::{Kinig, Neina, Obo, Ochta, Siebma, Unto, Zehna, Ass};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardNumber {
    Siebma = 7,
    Ochta = 8,
    Neina = 9,
    Zehna = 10,
    Unto = 11,
    Obo = 12,
    Kinig = 13,
    Ass = 14
}

const ARRAY: [CardNumber; 8] = [
    Siebma,
    Ochta,
    Neina,
    Zehna,
    Unto,
    Obo,
    Kinig,
    Ass
];

impl CardNumber {
    /// how the fuck do I get the value out of this enum
    fn index(&self) -> usize {
        ARRAY.iter().position(|&n| n == *self).unwrap()
    }

    fn diff(&self, other: &CardNumber) -> u16 {
        self.rec_diff(other, 0)
    }

    fn rec_diff(&self, other: &CardNumber, steps: u16) -> u16 {
        if self == other {
            return steps
        }
        let index = self.index();
        let next = if index + 1 == ARRAY.len() {
            ARRAY[0]
        } else {
            ARRAY[index + 1]
        };

        next.rec_diff(other, steps + 1)
    }

    pub(crate) fn is_somewhat_higher(&self, other: &CardNumber, distance: u16) -> bool {
        let diff = other.diff(&self);
        diff <= distance
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::card_number::CardNumber::{Ass, Neina, Ochta, Siebma, Zehna};

    #[test]
    fn diff() {
        assert!(Siebma.diff(&Siebma) == 0);
        assert!(Siebma.diff(&Ochta) == 1);
        assert!(Siebma.diff(&Neina) == 2);
        assert!(Ass.diff(&Siebma) == 1);
        assert!(Ass.diff(&Ochta) == 2);
    }

    #[test]
    fn is_somewhat_higher() {
        assert!(Ochta.is_somewhat_higher(&Siebma, 1));
        assert!(!Neina.is_somewhat_higher(&Siebma, 1));
        assert!(Neina.is_somewhat_higher(&Siebma, 2));
        assert!(!Zehna.is_somewhat_higher(&Siebma, 2));

        assert!(Siebma.is_somewhat_higher(&Ass, 1));
        assert!(!Ochta.is_somewhat_higher(&Ass, 1));
        assert!(Ochta.is_somewhat_higher(&Ass, 2));
        assert!(!Neina.is_somewhat_higher(&Ass, 2));
    }
}