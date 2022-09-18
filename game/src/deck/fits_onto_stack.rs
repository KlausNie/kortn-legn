use crate::deck::card::Card;

impl Card {
    pub fn fits_onto_stack(&self, stack: &Vec<Card>) -> bool {
        let last_on_stack = stack.last();
        if last_on_stack.is_none() {
            return true;
        }
        return self.is_next_higher(last_on_stack.unwrap())
    }

    pub fn fits_somewhat_onto_stack(&self, stack: &Vec<Card>, max_distance: u16) -> bool {
        let last_on_stack = stack.last();
        if last_on_stack.is_none() {
            return true;
        }
        return last_on_stack.unwrap().is_higher_or_less_than(self, max_distance)
    }
}