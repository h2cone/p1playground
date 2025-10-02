use crate::world::LEVELS_LEN;

#[derive(Copy, Clone)]
pub struct LevelNeighbors {
    pub left: Option<usize>,
    pub right: Option<usize>,
}

impl LevelNeighbors {
    pub fn for_level(index: usize) -> Self {
        if index >= LEVELS_LEN {
            return Self {
                left: None,
                right: None,
            };
        }

        let left = if index == 0 { None } else { Some(index - 1) };
        let right = if index + 1 < LEVELS_LEN {
            Some(index + 1)
        } else {
            None
        };

        Self { left, right }
    }
}
