use game::field::field::Field;
use game::field::play_source::PlaySource;
use game::field::play_source::PlaySource::{BottomStack1, BottomStack2, BottomStack3, NotPlayedCards};
use game::field::play_target::PlayTarget;
use game::field::play_target::PlayTarget::{TopStack1, TopStack2};
use crate::strategy::strategy::{Strategy, StrategyResult};

const COMBINATIONS: [(PlaySource, PlayTarget); 11] = [
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

pub struct SomewhatFittingPlay {}

impl Strategy for SomewhatFittingPlay {
    fn invoke(&self, field: &Field) -> StrategyResult {
        for (source, target) in COMBINATIONS {
            if let Some(card) = field.get_top_of_source(source) {
                if target == PlayTarget::BottomStack1 || target == PlayTarget::BottomStack2 || target == PlayTarget::BottomStack3 {
                    if card.fits_somewhat_onto_stack(field.get_target(target), 1) {
                        return StrategyResult::Play(source, target)
                    }
                }
                if card.fits_onto_stack(field.get_target(target)) {
                    return StrategyResult::Play(source, target)
                }
            }
        }

        return StrategyResult::None;
    }
}