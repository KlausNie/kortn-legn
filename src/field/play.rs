use std::collections::VecDeque;
use crate::{Card, Field};

pub enum PlaySource {
    BottomStack1,
    BottomStack2,
    BottomStack3,
    NotPlayedCards,
}

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
                let splits = self.not_played_cards.split_off(1);
                let first = self.not_played_cards.front().unwrap().clone();
                let rest = splits.as_slices().1.to_vec().clone();

                return match target {
                    PlayTarget::TopStack1 => {
                        self.top_stack1.push(first);
                        Field {
                            top_stack1: self.top_stack1.clone(),
                            top_stack2: self.top_stack2.clone(),
                            bottom_stack1: self.bottom_stack1.clone(),
                            bottom_stack2: self.bottom_stack2.clone(),
                            bottom_stack3: self.bottom_stack3.clone(),
                            not_played_cards: VecDeque::from(rest),
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
                            not_played_cards: VecDeque::from(rest),
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
                            not_played_cards: VecDeque::from(rest),
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
                            not_played_cards: VecDeque::from(rest),
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
                            not_played_cards: VecDeque::from(rest),
                        }
                    }
                }
            }
        };
    }

    fn can_play(&self, source: PlaySource, target: PlayTarget) -> bool {
        todo!()
    }
}