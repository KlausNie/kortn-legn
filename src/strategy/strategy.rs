use game::field::field::Field;
use game::field::play_source::PlaySource;
use game::field::play_target::PlayTarget;

pub enum StrategyResult {
    Play(PlaySource, PlayTarget),
    None
}

pub trait Strategy {
    fn invoke(&self, field: &Field) -> StrategyResult;
}