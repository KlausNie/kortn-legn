use crate::field::field::Field;

trait Playable {
    fn finished(&self) -> bool;
}

impl Playable for Field {
    fn finished(&self) -> bool {
        return self.top_stack1.len() + self.top_stack2.len() == 32;
    }
}