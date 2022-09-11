use rand::{Rng, thread_rng};
use crate::field::field::Field;
use crate::field::play_source::PlaySource;
use crate::field::play_target::PlayTarget;
use crate::strategy::strategy::{Strategy, StrategyResult};

pub struct RandomPlay {}

const SOURCES: [PlaySource; 4] = [
    PlaySource::NotPlayedCards,
    PlaySource::BottomStack1,
    PlaySource::BottomStack2,
    PlaySource::BottomStack3,
];
const TARGETS: [PlayTarget; 5] = [
    PlayTarget::BottomStack1,
    PlayTarget::BottomStack2,
    PlayTarget::BottomStack3,
    PlayTarget::TopStack1,
    PlayTarget::TopStack2,
];

impl Strategy for RandomPlay {
    fn invoke(&self, _field: &Field) -> StrategyResult {
        let source = SOURCES[random_nr_0_to(SOURCES.len())];
        let target = TARGETS[random_nr_0_to(TARGETS.len())];

        StrategyResult::Play(source, target)
    }
}


fn random_nr_0_to(max: usize) -> usize {
    let mut rng = thread_rng();
    return rng.gen_range(0..max);
}