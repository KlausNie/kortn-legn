use std::collections::VecDeque;
use crate::{Field};
use crate::deck::next_higher::NextHigher;
use crate::PlaySource::{BottomStack1, BottomStack2, BottomStack3, NotPlayedCards};
use crate::PlayTarget::{TopStack1, TopStack2};

#[derive(Debug,Copy, Clone, PartialEq)]
pub enum PlaySource {
    BottomStack1,
    BottomStack2,
    BottomStack3,
    NotPlayedCards,
}

#[derive(Debug,Copy, Clone, PartialEq)]
pub enum PlayTarget {
    TopStack1,
    TopStack2,
    BottomStack1,
    BottomStack2,
    BottomStack3,
}

pub trait Play {
    fn play_card(&mut self, source: PlaySource, target: PlayTarget) -> Field;

    fn can_play(&self, source: PlaySource, target: PlayTarget) -> bool;
}


impl Play for Field {
    fn play_card(&mut self, source: PlaySource, target: PlayTarget) -> Field {
        match source {
            PlaySource::BottomStack1 => {
                let first = self.bottom_stack1.first().unwrap().clone();
                let rest = self.bottom_stack1.split_off(1);

                match target {
                    PlayTarget::TopStack1 => {
                        self.top_stack1.push(first);
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: rest,
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    PlayTarget::TopStack2 => {
                        self.top_stack2.push(first);
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
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
                let first = self.bottom_stack2.first().unwrap().clone();
                let rest = self.bottom_stack2.split_off(1);

                match target {
                    PlayTarget::TopStack1 => {
                        self.top_stack1.push(first);
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: rest,
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    PlayTarget::TopStack2 => {
                        self.top_stack2.push(first);
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
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
                let rest = self.bottom_stack3.split_off(1);
                let first = self.bottom_stack1.first().unwrap().clone();

                match target {
                    PlayTarget::TopStack1 => {
                        self.top_stack1.push(first);
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: rest,
                            not_played_cards: self.not_played_cards.clone(),
                        };
                    }
                    PlayTarget::TopStack2 => {
                        self.top_stack2.push(first);
                        return Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
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
                let rest = self.not_played_cards.split_off(1);
                let first = self.not_played_cards.first().unwrap().clone();

                return match target {
                    PlayTarget::TopStack1 => {
                        self.top_stack1.push(first);
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::TopStack2 => {
                        self.top_stack2.push(first);
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::BottomStack1 => {
                        self.bottom_stack1.push(first);
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::BottomStack2 => {
                        self.bottom_stack2.push(first);
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: rest,
                        }
                    }
                    PlayTarget::BottomStack3 => {
                        self.bottom_stack3.push(first);
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
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
                self.bottom_stack1.first()
            }
            BottomStack2 => {
                self.bottom_stack2.first()
            }
            BottomStack3 => {
                self.bottom_stack3.first()
            }
            NotPlayedCards => {
                self.not_played_cards.first()
            }
        };

        let not_allowed =
            (
                source == BottomStack1 ||
                source == BottomStack2 ||
                source == BottomStack3
            ) && (
                source == BottomStack1 ||
                source == BottomStack2 ||
                source == BottomStack3
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
                    let current_top = current_top_wrapped.unwrap().clone();
                    println!("---{:?} {:?} {:?}", current_top, unwrapped_card, unwrapped_card.is_next_higher(current_top));
                    unwrapped_card.is_next_higher(current_top)
                }
            }
            TopStack2 => {
                let current_top_wrapped = self.top_stack2.last();
                if current_top_wrapped.is_none() {
                    true
                } else {
                    let current_top = current_top_wrapped.unwrap().clone();
                    println!("---{:?} {:?} {:?}", current_top, unwrapped_card, unwrapped_card.is_next_higher(current_top));
                    unwrapped_card.is_next_higher(current_top)
                }
            }
            _ => {
                true
            }
        };
    }
}