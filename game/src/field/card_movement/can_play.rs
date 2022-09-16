use crate::field::field::Field;
use crate::field::play_source::PlaySource;
use crate::field::play_source::PlaySource::{BottomStack1, BottomStack2, BottomStack3};
use crate::field::play_target::PlayTarget;

impl Field {
    pub(crate) fn can_play(&self, source: PlaySource, target: PlayTarget) -> bool {
        let card_to_be_played = match source {
            BottomStack1 => {
                self.bottom_stack1.last()
            }
            BottomStack2 => {
                self.bottom_stack2.last()
            }
            BottomStack3 => {
                self.bottom_stack3.last()
            }
            PlaySource::NotPlayedCards => {
                self.not_played_cards.last()
            }
        };

        let not_allowed =
            (
                source == BottomStack1 ||
                    source == BottomStack2 ||
                    source == BottomStack3
            ) && (
                target == PlayTarget::BottomStack1 ||
                    target == PlayTarget::BottomStack2 ||
                    target == PlayTarget::BottomStack3
            );

        if card_to_be_played.is_none() || not_allowed {
            return false
        }

        let unwrapped_card = card_to_be_played.unwrap();

        return match target {
            PlayTarget::TopStack1 => {
                let current_top_wrapped = self.top_stack1.last();
                if current_top_wrapped.is_none() {
                    true
                } else {
                    let current_top = current_top_wrapped.unwrap();
                    unwrapped_card.is_next_higher(current_top)
                }
            }
            PlayTarget::TopStack2 => {
                let current_top_wrapped = self.top_stack2.last();
                if current_top_wrapped.is_none() {
                    true
                } else {
                    let current_top = current_top_wrapped.unwrap();
                    unwrapped_card.is_next_higher(current_top)
                }
            }
            _ => {
                true
            }
        };
    }
}