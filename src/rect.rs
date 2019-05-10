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

    pub fn encompass_pos(&mut self, x: usize, y: usize) {
        self.x_range.start = usize::min(self.x_range.start, x);
        self.x_range.end = usize::max(self.x_range.end, x + 1);

        self.y_range.start = usize::min(self.y_range.start, y);
        self.y_range.end = usize::max(self.y_range.end, y + 1);
    }
}
