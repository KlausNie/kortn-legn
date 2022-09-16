use crate::field::field::Field;
use crate::field::play_source::PlaySource;
use crate::field::play_target::PlayTarget;

#[derive(Debug, PartialEq)]
pub enum FailedToPlay {
    InvalidMove(PlaySource, PlayTarget),
}

impl Field {
    pub fn try_play_card(&self, source: PlaySource, target: PlayTarget) -> Result<Field, FailedToPlay> {
        let valid_play = self.can_play(source, target);
        if !valid_play {
            return Err(FailedToPlay::InvalidMove(source, target))
        }

        return Ok(self.play_card(source, target));
    }
}