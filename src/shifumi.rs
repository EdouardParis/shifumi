#[derive(Clone, PartialEq)]
pub enum Play {
    Scissors,
    Rock,
    Paper,
}

impl Play {
    pub fn new() -> Play {
        Play::Paper
    }

    pub fn beat(&self) -> Play {
        match *self {
            Play::Paper => Play::Rock,
            Play::Rock => Play::Scissors,
            Play::Scissors => Play::Paper,
        }
    }

    pub fn lose(&self) -> Play {
        match *self {
            Play::Paper => Play::Scissors,
            Play::Rock => Play::Paper,
            Play::Scissors => Play::Rock,
        }
    }
}

pub enum Tree {
    Empty,
    Leaf {
        scissors: Box<Tree>,
        paper: Box<Tree>,
        rock: Box<Tree>,
    },
}

impl Tree {
    pub fn new() -> Tree {
        Tree::Leaf {
            scissors: Box::new(Tree::Empty),
            paper: Box::new(Tree::Empty),
            rock: Box::new(Tree::Empty),
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            Tree::Empty => true,
            _ => false,
        }
    }

    pub fn update(&mut self, plays: &[Play]) {
        self.update_leafs(plays);
        if let Some((_, rest)) = plays.split_first() {
            self.update(rest);
        }
    }

    fn update_leafs(&mut self, plays: &[Play]) {
        match plays.split_first() {
            Some((first, rest)) => {
                match *self {
                    Tree::Empty => {
                        *self = Tree::new();
                    }
                    Tree::Leaf {
                        ref mut scissors,
                        ref mut paper,
                        ref mut rock,
                    } => {
                        match *first {
                            Play::Scissors => scissors.update_leafs(rest),
                            Play::Paper => paper.update_leafs(rest),
                            Play::Rock => rock.update_leafs(rest),
                        }
                    }
                }
            }
            None => {
                match *self {
                    Tree::Empty => *self = Tree::new(),
                    _ => (),
                }
            }
        }
    }

    pub fn predict(&self, plays: &[Play]) -> Option<Play> {
        match plays.split_first() {
            Some((first, rest)) => {
                match *self {
                    Tree::Empty => None,
                    Tree::Leaf {
                        ref scissors,
                        ref paper,
                        ref rock,
                    } => {
                        let prediction = match *first {
                            Play::Scissors => scissors.predict(rest),
                            Play::Paper => paper.predict(rest),
                            Play::Rock => rock.predict(rest),
                        };
                        match prediction {
                            Some(play) => Some(play),
                            None => self.predict(rest),
                        }
                    }
                }
            }
            None => {
                match *self {
                    Tree::Empty => Some(Play::new()),
                    Tree::Leaf {
                        ref scissors,
                        ref paper,
                        ref rock,
                    } => {
                        if !scissors.is_empty() {
                            return Some(Play::Rock);
                        } else if !paper.is_empty() {
                            return Some(Play::Scissors);
                        } else if !rock.is_empty() {
                            return Some(Play::Paper);
                        }

                        return None;
                    }
                }
            }
        }
    }
}
