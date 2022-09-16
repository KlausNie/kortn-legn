extern crate core;
extern crate game;

mod play;
mod strategy;

use std::ops::{Add, Div, Mul};
use std::time::{Duration, Instant};
use game::deck::ordered::Ordered;
use game::field::overall_game_state::OverallGameState;
use crate::play::playing::playing;
use crate::strategy::random_play::RandomPlay;
use crate::strategy::simple_best_play::SimpleBestPlay;

fn main() {
    let n = 1000;
    let (wins, avg_duration, avg_steps_when_winning, avg_steps_when_stuck) = play_games(n, false);
    println!("{:?}/{:?} = {:.2}%", wins, n, (wins as f64).div(n as f64).mul(100.0));
    println!("avg duration: {:?}", avg_duration);
    println!("(win)   avg steps: {:?}", avg_steps_when_winning);
    println!("(stuck) avg steps: {:?}", avg_steps_when_stuck);
}

fn play_games(n: i32, log: bool) -> (i32, Duration, Option<u32>, Option<u32>) {
    let mut wins = 0;
    let mut total_duration: Duration = Duration::new(0, 0);
    let mut steps_when_winning: u32 = 0;
    let mut steps_when_stuck: u32 = 0;
    for i in 0..n {
        let start = Instant::now();
        let result_field = playing(SimpleBestPlay{});
        if result_field.finished() == OverallGameState::Success {
            wins = wins + 1;
            steps_when_winning = steps_when_winning + result_field.stats.steps;
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
            steps_when_stuck = steps_when_stuck + result_field.stats.steps;
        }
        total_duration = total_duration.add(start.elapsed());
    }
    let duration = total_duration.div(n as u32);
    let i1 = steps_when_winning.checked_div(wins as u32);
    let i2 = steps_when_stuck.checked_div((n - wins) as u32);
    (wins, duration, i1, i2)
}