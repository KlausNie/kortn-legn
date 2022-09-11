use crate::Card;

pub trait Ordered {
    fn is_in_order(&self) -> bool;
}

impl Ordered for Vec<Card> {
    fn is_in_order(&self) -> bool {
        let mut is_sorted_incrementally = true;
        if self.len() <= 0 {
            return true
        }

        let indexed = 0..self.len() - 1;
        for index in indexed {
            let array_elem1 = &self[index];
            let array_elem2 = self[index + 1];
            if !array_elem2.is_next_higher(array_elem1) {
                is_sorted_incrementally = false;
            }
        }

        return is_sorted_incrementally;
    }
}