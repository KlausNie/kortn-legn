use rand::{Rng, thread_rng};
use crate::deck::card::Card;
use crate::deck::create_deck::create_deck;
use crate::deck::randomize::Randomize;
use crate::field::best_play::HasBestPlay;
use crate::field::field::Field;
use crate::field::init_field::init_field;
use crate::field::play_source::PlaySource;
use crate::field::playable::Playable;
use crate::field::overall_game_state::OverallGameState;
use crate::field::play_target::PlayTarget;

fn setup() -> Field {
    let deck: Vec<Card> = create_deck();
    let shuffled = deck.randomize();
    init_field(shuffled)
}

fn random_nr_0_to(max: usize) -> usize {
    let mut rng = thread_rng();
    return rng.gen_range(0..max);
}

pub fn play() -> Field {
    let mut field = setup();

    let sources: [PlaySource; 4] = [
        PlaySource::NotPlayedCards,
        PlaySource::BottomStack1,
        PlaySource::BottomStack2,
        PlaySource::BottomStack3,
    ];
    let targets: [PlayTarget; 5] = [
        PlayTarget::BottomStack1,
        PlayTarget::BottomStack2,
        PlayTarget::BottomStack3,
        PlayTarget::TopStack1,
        PlayTarget::TopStack2,
    ];

    while field.finished() == OverallGameState::NotYetDone {

        let (source, target) = match field.best_play() {
            HasBestPlay::BestPlay(source, target) => {
                (source, target)
            }
            HasBestPlay::None => {
                let source = sources[random_nr_0_to(sources.len())];
                let target = targets[random_nr_0_to(targets.len())];

                (source, target)
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