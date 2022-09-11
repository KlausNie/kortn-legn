use crate::field::field::Field;
use crate::{Ordered};
use crate::field::overall_game_state::OverallGameState;
use crate::field::overall_game_state::OverallGameState::{NotYetDone, Stuck, Success};
use crate::field::play_source::PlaySource::{BottomStack1, BottomStack2, BottomStack3, NotPlayedCards};
use crate::field::play_target::PlayTarget;
use crate::field::play_target::PlayTarget::{TopStack1, TopStack2};

pub trait Playable {
    fn finished(&self) -> OverallGameState;
}

impl Playable for Field {
    /// consider unifying function (since it is so similar) with best_play.rs
    fn finished(&self) -> OverallGameState {
        let all_cards_played = self.not_played_cards.len() == 0;

        if !all_cards_played {
            return NotYetDone
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
                return NotYetDone
            }
        }

        if self.top_stack1.len() + self.top_stack2.len() == 32 {
            let top_stack1_in_order = self.top_stack1.is_in_order();
            let top_stack2_in_order = self.top_stack2.is_in_order();
            if top_stack1_in_order &&
                top_stack2_in_order {
                return Success
            }
        }

        return Stuck
    }
}