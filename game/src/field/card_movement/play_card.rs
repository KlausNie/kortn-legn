use crate::field::field::{Field};
use crate::field::play_source::PlaySource;
use crate::field::play_target::PlayTarget;
use crate::field::stats::{Stats, Step};

impl Field {
    /// TODO investigate if all these clones are necessary, or if I can do it with references
    /// TODO would be cool if this could be simplified by A LOT
    ///
    /// Was looking for a way to copy and change only a few fields, found struct deconstruction like `..Struct` is
    /// possible, but not if the struct contains Vec<T> because that renders the whole struct not copiable anymore
    /// (copy only for bitwise copies 🤷‍). Since I cannot implement a Copy for that I considered using
    /// slices for the fields in the struct, but slices do not allow appending, and I'm not going down the
    /// route to provide a mutable slice reference...
    ///
    ///
    /// All I'm doing here, is picking a card from a stack and putting it onto another one
    pub(crate) fn play_card(&self, source: PlaySource, target: PlayTarget) -> Field {
        let (first, rest) = self.get_top_and_rest_of_source(source);
        let first = first.unwrap().clone();
        let mut steps = self.stats.steps.clone();
        steps.push_back(Step::from(source, target, first.clone()));

        match source {
            PlaySource::BottomStack1 => {
                match target {
                    PlayTarget::TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: rest,
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
                        };
                    }
                    _ => { panic!() }
                }
            }
            PlaySource::BottomStack2 => {
                match target {
                    PlayTarget::TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: rest,
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
                        };
                    }
                    _ => { panic!() }
                }
            }
            PlaySource::BottomStack3 => {
                match target {
                    PlayTarget::TopStack1 => {
                        return Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: rest,
                            not_played_cards: self.not_played_cards.clone(),
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
                        };
                    }
                    _ => { panic!() }
                }
            }
            PlaySource::NotPlayedCards => {
                return match target {
                    PlayTarget::TopStack1 => {
                        Field {
                            top_stack1: clone_and_add(&self.top_stack1, first),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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
                            stats: Stats {
                                steps: steps,
                                ..self.stats
                            }
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