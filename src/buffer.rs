use cursive::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetResult {
    OutOfBounds,
    DifferentValue,
    SameValue,
}

pub struct Buffer<T>
where
    T: Clone + PartialEq,
{
    pub buffer: Vec<T>,
    pub size: Vec2,
}

impl<T> Buffer<T>
where
    T: Clone + PartialEq,
{
    pub fn new(size: Vec2, value: T) -> Self {
        let mut buffer: Vec<T> = Vec::new();
        buffer.resize(size.x * size.y, value);
        Buffer { buffer, size }
    }

    pub fn resize(&mut self, size: Vec2, value: T) {
        self.buffer.resize(size.x * size.y, value);
        self.size = size;
    }

    pub fn get_item(&self, x: usize, y: usize) -> &T {
        &self.buffer[y * self.size.x + x]
    }

    pub fn set_item(&mut self, x: usize, y: usize, new_value: T) -> SetResult {
        if x >= self.size.x || y >= self.size.y {
            SetResult::OutOfBounds
        } else {
            let pos = y * self.size.x + x;
            let old_value = &self.buffer[pos];
            if old_value == &new_value {
                SetResult::SameValue
            } else {
                self.buffer[pos] = new_value;
                SetResult::DifferentValue
            }
        }
    }
}
