use crate::deck::next_higher::NextHigher;
use crate::field::field::Field;
use crate::PlaySource::{BottomStack1, BottomStack2, BottomStack3, NotPlayedCards};
use crate::PlayTarget;
use crate::PlayTarget::{TopStack1, TopStack2};

pub trait Playable {
    fn finished(&self) -> bool;
}

impl Playable for Field {
    /// consider unifying with best_play.rs
    fn finished(&self) -> bool {
        let all_cards_played = self.not_played_cards.len() == 0;

        if !all_cards_played {
            return false
        }

        let combinations = [
            (BottomStack1, TopStack1),
            (BottomStack1, TopStack2),
            (BottomStack2, TopStack1),
            (BottomStack2, TopStack2),
            (BottomStack3, TopStack1),
            (BottomStack3, TopStack2),
            (NotPlayedCards, TopStack1),
            (NotPlayedCards, TopStack2),
            (NotPlayedCards, PlayTarget::BottomStack1),
            (NotPlayedCards, PlayTarget::BottomStack2),
            (NotPlayedCards, PlayTarget::BottomStack3),
        ];

        for (source, target) in combinations {
            let card = self.get_top_of_source(source);
            if card.is_some() && card.unwrap().fits_onto_stack(self.get_target(target)) {
                return false
            }
        }

        return true
    }
}