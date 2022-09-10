use rand::{Rng, thread_rng};
use crate::deck::card::Card;

pub trait Randomize<T> {
    fn randomize(&self) -> T;
}

impl Randomize<Vec<Card>> for Vec<Card> {
    fn randomize(&self) -> Vec<Card> {
        let mut rng = thread_rng();
        let size = self.len();

        let mut ret = self.clone();
        for _i in 0..100 {
            let first: usize = rng.gen_range(0..size);
            let second: usize = rng.gen_range(0..size);
            ret.swap(first,second);
        }

        ret
    }
}