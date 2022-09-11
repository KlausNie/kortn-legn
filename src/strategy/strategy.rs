use crate::field::field::Field;
use crate::field::play_source::PlaySource;
use crate::field::play_target::PlayTarget;

pub enum StrategyResult {
    Play(PlaySource, PlayTarget),
    None
}

pub trait Strategy {
    fn invoke(&self, field: &Field) -> StrategyResult;
}