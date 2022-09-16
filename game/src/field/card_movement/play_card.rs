use crate::field::field::Field;
use crate::field::play_source::PlaySource;
use crate::field::play_target::PlayTarget;

impl Field {
    /// TODO investigate if all these clones are necessary, or if I can do it with references
    /// TODO would be cool if this could be simplified by A LOT
    pub(crate) fn play_card(&self, source: PlaySource, target: PlayTarget) -> Field {
        match source {
            PlaySource::BottomStack1 => {
                let (first, rest) = self.get_top_and_rest_of_source(PlaySource::BottomStack1);
                let first = first.unwrap().clone();

                match target {
                    PlayTarget::TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: rest,
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    PlayTarget::TopStack2 => {
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
            PlaySource::BottomStack2 => {
                let (first, rest) = self.get_top_and_rest_of_source(PlaySource::BottomStack2);
                let first = first.unwrap().clone();

                match target {
                    PlayTarget::TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: rest,
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    PlayTarget::TopStack2 => {
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
            PlaySource::BottomStack3 => {
                let (first, rest) = self.get_top_and_rest_of_source(PlaySource::BottomStack3);
                let first = first.unwrap().clone();

                match target {
                    PlayTarget::TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: rest,
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    PlayTarget::TopStack2 => {
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
            PlaySource::NotPlayedCards => {
                let (first, rest) = self.get_top_and_rest_of_source(PlaySource::NotPlayedCards);
                let first = first.unwrap().clone();

                return match target {
                    PlayTarget::TopStack1 => {
                        Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::TopStack2 => {
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
}


fn clone_and_add<T : Clone>(vec: &Vec<T>, item: T) -> Vec<T> {
    let mut new_vec = vec.clone();
    new_vec.push(item);
    new_vec
}