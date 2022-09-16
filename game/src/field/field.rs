use crate::deck::card::Card;
use crate::field::play_source::PlaySource;
use crate::field::play_target::PlayTarget;

#[derive(Debug)]
pub struct Field {
    pub top_stack1: Vec<Card>,
    pub top_stack2: Vec<Card>,
    pub bottom_stack1: Vec<Card>,
    pub bottom_stack2: Vec<Card>,
    pub bottom_stack3: Vec<Card>,
    pub not_played_cards: Vec<Card>,
    pub stats: Stats
}

#[derive(Debug, Copy, Clone)]
pub struct Stats {
    pub steps: u32
}

impl Field {
    pub fn get_source(&self, play_source: PlaySource) -> &Vec<Card> {
        return match play_source {
            PlaySource::BottomStack1 => {
                &self.bottom_stack1
            }
            PlaySource::BottomStack2 => {
                &self.bottom_stack2
            }
            PlaySource::BottomStack3 => {
                &self.bottom_stack3
            }
            PlaySource::NotPlayedCards => {
                &self.not_played_cards
            }
        }
    }

    pub fn get_target(&self, play_target: PlayTarget) -> &Vec<Card> {
        return match play_target {
            PlayTarget::TopStack1 => {
                &self.top_stack1
            }
            PlayTarget::TopStack2 => {
                &self.top_stack2
            }
            PlayTarget::BottomStack1 => {
                &self.bottom_stack1
            }
            PlayTarget::BottomStack2 => {
                &self.bottom_stack2
            }
            PlayTarget::BottomStack3 => {
                &self.bottom_stack3
            }
        }
    }

    pub fn get_top_of_source(&self, play_source: PlaySource) -> Option<&Card> {
        self.get_source(play_source).last()
    }

    pub fn get_top_and_rest_of_source(&self, play_source: PlaySource) -> (Option<&Card>, Vec<Card>) {
        let head = self.get_top_of_source(play_source);
        let source_list = self.get_source(play_source);
        let rest = (source_list[..source_list.len() - 1]).to_vec();

        (head, rest)
    }

    pub fn stats(&self) -> String {
        return format!(
            "t1: {:?}  t2: {:?}  b1: {:?}  b2: {:?}  b3: {:?}",
            self.top_stack1.len(),
            self.top_stack2.len(),
            self.bottom_stack1.len(),
            self.bottom_stack2.len(),
            self.bottom_stack3.len()
        )
    }
}