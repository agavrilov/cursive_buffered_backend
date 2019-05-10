use std::ops::Range;
use std::usize;

#[derive(Clone)]
pub struct Rect {
    pub x_range: Range<usize>,
    pub y_range: Range<usize>,
}

const EMPTY_RANGE: Range<usize> = usize::MAX..usize::MIN;

impl Rect {
    pub fn new() -> Self {
        Rect {
            x_range: EMPTY_RANGE,
            y_range: EMPTY_RANGE,
        }
    }

    pub fn reset(&mut self) {
        self.x_range = EMPTY_RANGE;
        self.y_range = EMPTY_RANGE;
    }
}
