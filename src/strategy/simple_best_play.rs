use game::field::field::Field;
use game::field::play_source::PlaySource::{BottomStack1, BottomStack2, BottomStack3, NotPlayedCards};
use game::field::play_target::PlayTarget;
use game::field::play_target::PlayTarget::{TopStack1, TopStack2};
use crate::strategy::strategy::{Strategy, StrategyResult};

pub struct SimpleBestPlay {}

impl Strategy for SimpleBestPlay {
    fn invoke(&self, field: &Field) -> StrategyResult {
        let combinations = [
            (BottomStack1, TopStack1),
            (BottomStack2, TopStack1),
            (BottomStack3, TopStack1),
            (BottomStack1, TopStack2),
            (BottomStack2, TopStack2),
            (BottomStack3, TopStack2),
            (NotPlayedCards, TopStack1),
            (NotPlayedCards, TopStack2),
            (NotPlayedCards, PlayTarget::BottomStack1),
            (NotPlayedCards, PlayTarget::BottomStack2),
            (NotPlayedCards, PlayTarget::BottomStack3),
        ];

        for (source, target) in combinations {
            let card = field.get_top_of_source(source);
            if card.is_some() && card.unwrap().fits_onto_stack(field.get_target(target)) {
                return StrategyResult::Play(source, target)
            }
        }

        return StrategyResult::None;
    }
}