extern crate core;
extern crate game;

mod play;
mod strategy;

use std::ops::{Add, Div, Mul};
use std::time::{Duration, Instant};
use game::deck::ordered::Ordered;
use game::field::overall_game_state::OverallGameState;
use crate::play::playing::playing;
use crate::strategy::simple_best_play::SimpleBestPlay;

fn main() {
    let n = 100000;
    let (wins, avg_duration) = play_games(n, false);
    println!("{:?}/{:?} = {:.2}%", wins, n, (wins as f64).div(n as f64).mul(100.0));
    println!("avg duration: {:?}", avg_duration)
}

fn play_games(n: i32, log: bool) -> (i32, Duration) {
    let mut wins = 0;
    let mut total_duration: Duration = Duration::new(0, 0);
    for i in 0..n {
        let start = Instant::now();
        let result_field = playing(SimpleBestPlay{});
        if result_field.finished() == OverallGameState::Success {
            wins = wins + 1;
            if log {
                println!("Game {:?} ended with result {:?} ({})", i, result_field.finished(), result_field.stats());
                println!("not_played_cards: {:?}", result_field.not_played_cards);
                println!("top_stack1: {:?} {:?}", result_field.top_stack1.is_in_order(), result_field.top_stack1);
                println!("top_stack2: {:?} {:?}", result_field.top_stack2.is_in_order(), result_field.top_stack2);
                println!("bottom_stack1: {:?}", result_field.bottom_stack1);
                println!("bottom_stack2: {:?}", result_field.bottom_stack2);
                println!("bottom_stack3: {:?}", result_field.bottom_stack3);
            }
        } else {
            if log {
                println!("Game {:?} ended with result {:?} ({})", i, result_field.finished(), result_field.stats())
            }
        }
        total_duration = total_duration.add(start.elapsed())
    }
    (wins, total_duration.div(n as u32))
}