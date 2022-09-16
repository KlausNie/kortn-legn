use crate::field::field::Field;
use crate::{OverallGameState};
use crate::play::setup::setup;
use crate::strategy::random_play::RandomPlay;
use crate::strategy::strategy::{Strategy, StrategyResult};

pub(crate) fn playing<T: Strategy>(strategy: T) -> Field {
    let mut field: Field = setup();
    let random_strategy = RandomPlay {};

    while field.finished() == OverallGameState::NotYetDone {

        let strategy_result = strategy.invoke(&field);
        let (source, target) = match strategy_result {
            StrategyResult::Play(source, target) => {
                (source, target)
            }
            StrategyResult::None => {
                let result = random_strategy.invoke(&field);
                match result {
                    StrategyResult::Play(source, target) => {
                        (source, target)
                    }
                    StrategyResult::None => {
                        panic!()
                    }
                }
            }
        };

        let result = field.try_play_card(source, target);
        if result.is_ok() {
            field = result.unwrap()
        } else {
            // println!("Error happened: {:?}", result.unwrap_err())
        }
    }

    return field;
}