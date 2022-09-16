use crate::deck::card::Card;

impl Card {
    pub fn fits_onto_stack(&self, stack: &Vec<Card>) -> bool {
        let last_on_stack = stack.last();
        if last_on_stack.is_none() {
            return true;
        }
        return self.is_next_higher(last_on_stack.unwrap())
    }
}