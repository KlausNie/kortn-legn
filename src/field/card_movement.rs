use crate::deck::next_higher::NextHigher;
use crate::field::field::Field;
use crate::field::play_source::PlaySource;
use crate::field::play_source::PlaySource::{BottomStack1, BottomStack2, BottomStack3};
use crate::field::play_target::PlayTarget;
use crate::field::play_target::PlayTarget::{TopStack1, TopStack2};

pub trait CardMovement {
    fn play_card(&self, source: PlaySource, target: PlayTarget) -> Field;

    fn can_play(&self, source: PlaySource, target: PlayTarget) -> bool;
}

fn clone_and_add<T : Clone>(vec: &Vec<T>, item: T) -> Vec<T> {
    let mut new_vec = vec.clone();
    new_vec.push(item);
    new_vec
}

impl CardMovement for Field {
    /// TODO investigate if all these clones are necessary, or if I can do it with references
    fn play_card(&self, source: PlaySource, target: PlayTarget) -> Field {
        match source {
            BottomStack1 => {
                let (first, rest) = self.get_top_and_rest_of_source(BottomStack1);
                let first = first.unwrap().clone();

                match target {
                    TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: rest,
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    TopStack2 => {
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: clone_and_add(&self.top_stack2, first),
                            bottom_stack1: rest,
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    _ => { panic!() }
                }
            }
            BottomStack2 => {
                let (first, rest) = self.get_top_and_rest_of_source(BottomStack2);
                let first = first.unwrap().clone();

                match target {
                    TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: rest,
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    TopStack2 => {
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: clone_and_add(&self.top_stack2, first),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: rest,
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    _ => { panic!() }
                }
            }
            BottomStack3 => {
                let (first, rest) = self.get_top_and_rest_of_source(BottomStack3);
                let first = first.unwrap().clone();

                match target {
                    TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: rest,
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    TopStack2 => {
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: clone_and_add(&self.top_stack2, first),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: rest,
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    _ => { panic!() }
                }
            }
            NotPlayedCards => {
                let (first, rest) = self.get_top_and_rest_of_source(NotPlayedCards);
                let first = first.unwrap().clone();

                return match target {
                    TopStack1 => {
                        Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    TopStack2 => {
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: clone_and_add(&self.top_stack2, first),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::BottomStack1 => {
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: clone_and_add(&self.bottom_stack1, first),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::BottomStack2 => {
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: clone_and_add(&self.bottom_stack2, first),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::BottomStack3 => {
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: clone_and_add(&self.bottom_stack3, first),
                            not_played_cards: rest,
                        }
                    }
                }
            }
        };
    }

    fn can_play(&self, source: PlaySource, target: PlayTarget) -> bool {
        let card_to_be_played = match source {
            BottomStack1 => {
                self.bottom_stack1.last()
            }
            BottomStack2 => {
                self.bottom_stack2.last()
            }
            BottomStack3 => {
                self.bottom_stack3.last()
            }
            NotPlayedCards => {
                self.not_played_cards.last()
            }
        };

        let not_allowed =
            (
                source == BottomStack1 ||
                source == BottomStack2 ||
                source == BottomStack3
            ) && (
                target == PlayTarget::BottomStack1 ||
                target == PlayTarget::BottomStack2 ||
                target == PlayTarget::BottomStack3
            );

        if card_to_be_played.is_none() || not_allowed {
            return false
        }

        let unwrapped_card = card_to_be_played.unwrap();

        return match target {
            TopStack1 => {
                let current_top_wrapped = self.top_stack1.last();
                if current_top_wrapped.is_none() {
                    true
                } else {
                    let current_top = current_top_wrapped.unwrap();
                    unwrapped_card.is_next_higher(current_top)
                }
            }
            TopStack2 => {
                let current_top_wrapped = self.top_stack2.last();
                if current_top_wrapped.is_none() {
                    true
                } else {
                    let current_top = current_top_wrapped.unwrap();
                    unwrapped_card.is_next_higher(current_top)
                }
            }
            _ => {
                true
            }
        };
    }
}