use crate::deck::next_higher::NextHigher;
use crate::field::field::Field;
use crate::field::play_source::PlaySource;
use crate::field::play_source::PlaySource::{BottomStack1, BottomStack2, BottomStack3, NotPlayedCards};
use crate::field::play_target::{PlayTarget};
use crate::field::play_target::PlayTarget::{TopStack1, TopStack2};

pub enum HasBestPlay {
    None,
    BestPlay(PlaySource, PlayTarget)
}

impl Field {
    /// consider unifying with playable.rs
    pub(crate) fn best_play(&self) -> HasBestPlay {
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
                return HasBestPlay::BestPlay(source, target)
            }
        }

        return HasBestPlay::None;
    }
}