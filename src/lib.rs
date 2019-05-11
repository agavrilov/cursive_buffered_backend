#[macro_use]
extern crate log;

use cursive::backend::Backend;
use cursive::event::Event;
use cursive::theme;
use cursive::Vec2;
use enumset::EnumSet;
use std::cell::{Cell, RefCell};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

mod smallstring;

use smallstring::SmallString;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Style {
    effects: EnumSet<theme::Effect>,
    color_pair: theme::ColorPair,
}

type StyledText = Option<(Style, SmallString)>;

pub struct BufferedBackend {
    /// original backend
    backend: Box<Backend>,

    /// the buffer to write a new content to
    write_buffer: RefCell<Vec<StyledText>>,

    /// Content on the screen
    read_buffer: RefCell<Vec<StyledText>>,

    /// the size of the screen
    size: Cell<Vec2>,

    /// the current style
    current_style: RefCell<Style>,
}

fn background_style() -> Style {
    Style {
        effects: EnumSet::new(),
        color_pair: theme::ColorPair {
            front: theme::Color::Dark(theme::BaseColor::Black),
            back: theme::Color::Dark(theme::BaseColor::Black),
        },
    }
}

fn default_styled_text() -> StyledText {
    Some((background_style(), " ".into()))
}

fn allocate_buffer(size: Vec2) -> Vec<StyledText> {
    let mut buffer: Vec<StyledText> = Vec::new();
    buffer.resize(size.x * size.y, default_styled_text());
    buffer
}

fn resize_buffer(buf: &mut Vec<StyledText>, size: Vec2, new_style: Style) {
    // first, resize the buffer to match the screen size
    let new_length = size.x * size.y;
    if buf.len() != new_length {
        buf.resize(new_length, default_styled_text());
    }

    // clear all cells
    for cell in buf.iter_mut() {
        match *cell {
            Some((ref mut style, ref mut text)) => {
                *style = new_style;
                text.clear();
                text.push_str(" ");
            }
            _ => {
                *cell = Some((new_style, " ".into()));
            }
        }
    }
}

fn write_effect(
    backend: &Backend,
    effects: &EnumSet<theme::Effect>,
    effect: theme::Effect,
    set: bool,
) {
    if effects.contains(effect) {
        if set {
            backend.set_effect(effect);
        } else {
            backend.unset_effect(effect);
        }
    }
}

fn write_effects(backend: &Backend, effects: &EnumSet<theme::Effect>, set: bool) {
    write_effect(backend, &effects, theme::Effect::Simple, set);
    write_effect(backend, &effects, theme::Effect::Reverse, set);
    write_effect(backend, &effects, theme::Effect::Bold, set);
    write_effect(backend, &effects, theme::Effect::Italic, set);
    write_effect(backend, &effects, theme::Effect::Underline, set);
}

impl BufferedBackend {
    pub fn new(backend: Box<Backend>) -> Self {
        let screen_size = backend.screen_size();
        BufferedBackend {
            backend,
            write_buffer: RefCell::new(allocate_buffer(screen_size)),
            read_buffer: RefCell::new(allocate_buffer(screen_size)),
            size: Cell::new(screen_size),
            current_style: RefCell::new(background_style()),
        }
    }

    fn resize_and_clear(&self, new_style: Style) {
        let screen_size = self.backend.screen_size();

        // clear write buffer
        {
            let mut buf = self.write_buffer.borrow_mut();
            resize_buffer(&mut buf, screen_size, new_style);
        }

        // clear read buffer
        {
            let mut buf = self.read_buffer.borrow_mut();
            resize_buffer(&mut buf, screen_size, new_style);
        }

        self.size.set(screen_size);
    }

    fn output_all_to_backend(&mut self) {
        debug!("output_all_to_backend started");
        {
            let write_buffer = self.write_buffer.borrow();
            let read_buffer = self.read_buffer.borrow();

            let mut last_style = background_style();

            let mut current_pos = Vec2::new(0, 0);
            let mut current_text = SmallString::new();
            let size = self.size.get();
            for y in 0..size.y {
                current_pos.x = 0;
                current_pos.y = y;
                current_text.clear();

                for x in 0..size.x {
                    let pos = y * size.x + x;
                    let old_value = &read_buffer[pos];
                    let new_value = &write_buffer[pos];

                    // if we have the same content on the screen (read buffer) as in the write buffer,
                    // skip it for output, but output the text already collected
                    if new_value == old_value {
                        self.output_to_backend(current_pos, &current_text, &last_style);
                        if let Some((style, _)) = new_value {
                            last_style = *style;
                        }
                        current_pos.x = x;
                        current_text.clear();
                    } else {
                        if let Some((style, ref text)) = new_value {
                            if *style != last_style {
                                self.output_to_backend(current_pos, &current_text, &last_style);

                                last_style = *style;
                                current_pos.x = x;
                                current_text.clear();
                            }

                            current_text.push_str(&text);
                        }
                    }
                }

                self.output_to_backend(current_pos, &current_text, &last_style);
            }
        }

        // Make sure everything is written out
        self.backend.refresh();

        // swap read and write buffers to compare against written content in future iterations
        self.write_buffer.swap(&self.read_buffer);

        debug!("output_all_to_backend finished");
    }

    fn output_to_backend(&self, pos: Vec2, text: &str, style: &Style) {
        if !text.is_empty() {
            trace!(
                "output_to_backend: pos={:?}, text={:?}, style={:?}",
                pos,
                text,
                style
            );
            write_effects(&*self.backend, &style.effects, true);
            self.backend.set_color(style.color_pair);
            self.backend.print_at(pos, &text);
            write_effects(&*self.backend, &style.effects, false);
        }
    }

    fn output_to_buffer(&self, x: usize, y: usize, text: &str, style: Style) {
        let size = self.size.get();
        if y < size.y {
            let mut buf = self.write_buffer.borrow_mut();
            let mut x = x;
            for g in UnicodeSegmentation::graphemes(text, true) {
                let width = UnicodeWidthStr::width(g);
                if width > 0 {
                    if x < size.x {
                        buf[y * size.x + x] = Some((style, g.into()));
                    }
                    x += 1;
                    for _ in 0..(width - 1) {
                        if x < size.x {
                            buf[y * size.x + x] = None;
                        }
                        x += 1;
                    }
                }
            }
        }
    }
}

impl Backend for BufferedBackend {
    fn poll_event(&mut self) -> Option<Event> {
        self.backend.poll_event()
    }

    // TODO: take `self` by value?
    // Or implement Drop?
    /// Prepares to close the backend.
    ///
    /// This should clear any state in the terminal.
    fn finish(&mut self) {
        trace!("Start finishing BufferedBackend");
        self.backend.finish();
        trace!("End finishing BufferedBackend");
    }

    /// Refresh the screen.
    fn refresh(&mut self) {
        self.output_all_to_backend();
    }

    /// Should return `true` if this backend supports colors.
    fn has_colors(&self) -> bool {
        self.backend.has_colors()
    }

    /// Returns the screen size.
    fn screen_size(&self) -> Vec2 {
        self.backend.screen_size()
    }

    /// Main method used for printing
    fn print_at(&self, pos: Vec2, text: &str) {
        self.output_to_buffer(pos.x, pos.y, text, *self.current_style.borrow());
    }

    /// Clears the screen with the given color.
    fn clear(&self, color: theme::Color) {
        let style = Style {
            effects: EnumSet::new(),
            color_pair: theme::ColorPair {
                front: color,
                back: color,
            },
        };
        self.resize_and_clear(style);
    }

    /// Starts using a new color.
    ///
    /// This should return the previously active color.
    fn set_color(&self, colors: theme::ColorPair) -> theme::ColorPair {
        let mut current_style = self.current_style.borrow_mut();
        let previous_colors = current_style.color_pair;
        current_style.color_pair = colors;
        previous_colors
    }

    /// Enables the given effect.
    fn set_effect(&self, effect: theme::Effect) {
        let mut current_style = self.current_style.borrow_mut();
        current_style.effects.insert(effect);
    }

    /// Disables the given effect.
    fn unset_effect(&self, effect: theme::Effect) {
        let mut current_style = self.current_style.borrow_mut();
        current_style.effects.remove(effect);
    }

    /// Returns a name to identify the backend.
    ///
    /// Mostly used for debugging.
    fn name(&self) -> &str {
        "buffered_backend"
    }
}
