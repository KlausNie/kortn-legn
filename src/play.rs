use crate::deck::card::Card;
use crate::deck::create_deck::create_deck;
use crate::deck::randomize::Randomize;
use crate::field::field::Field;
use crate::field::init_field::init_field;
use crate::field::overall_game_state::OverallGameState;
use crate::strategy::random_play::RandomPlay;
use crate::strategy::strategy::{Strategy, StrategyResult};

fn setup() -> Field {
    let deck: Vec<Card> = create_deck();
    let shuffled = deck.randomize();
    init_field(shuffled)
}


pub fn play<T: Strategy>(strategy: T) -> Field {
    let mut field = setup();
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