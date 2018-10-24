extern crate cursive;
extern crate enumset;
extern crate smallvec;
extern crate unicode_segmentation;
extern crate unicode_width;

use cursive::backend::Backend;
use cursive::theme::{ColorStyle, Style};
use cursive::Vec2;

use std::cell::RefCell;

mod smallstring;

use enumset::EnumSet;
use smallstring::SmallString;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub struct BufferedBackend {
    backend: Box<Backend>,
    buf: RefCell<Vec<Option<(Style, SmallString)>>>,
    w: usize,
    h: usize,
}

fn background_style() -> Style {
    Style {
        effects: EnumSet::new(),
        color: Some(ColorStyle::background()),
    }
}

fn write_style(backend: &Backend, style: &Style) {}

impl BufferedBackend {
    pub fn new(backend: Box<Backend>) -> Self {
        let screen_size = backend.screen_size();
        let w = screen_size.x;
        let h = screen_size.y;
        let style = background_style();
        let buf = std::iter::repeat(Some((style, " ".into())))
            .take(w as usize * h as usize)
            .collect();
        BufferedBackend {
            backend,
            buf: RefCell::new(buf),
            w: w as usize,
            h: h as usize,
        }
    }

    fn clear(&self, new_style: Style) {
        for cell in self.buf.borrow_mut().iter_mut() {
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

    fn resize(&mut self, w: usize, h: usize) {
        self.w = w;
        self.h = h;
        self.buf
            .borrow_mut()
            .resize(w * h, Some((background_style(), " ".into())));
    }

    fn present(&mut self) {
        let buf = self.buf.borrow();

        let mut last_style = background_style();
        write_style(&*self.backend, &last_style);

        let mut pos = Vec2::new(0, 0);
        while pos.y < self.h {
            pos.x = 0;
            while pos.x < self.w {
                if let Some((style, ref text)) = buf[pos.y * self.w + pos.x] {
                    if style != last_style {
                        write_style(&*self.backend, &style);
                        last_style = style;
                    }
                    self.backend.print_at(pos, text);
                }
                pos.x += 1;
            }
            pos.y += 1;
        }

        // Make sure everything is written out
        self.backend.refresh();
    }

    fn draw(&self, x: usize, y: usize, text: &str, style: Style) {
        if y < self.h {
            let mut buf = self.buf.borrow_mut();
            let mut x = x;
            for g in UnicodeSegmentation::graphemes(text, true) {
                let width = UnicodeWidthStr::width(g);
                if width > 0 {
                    if x < self.w {
                        buf[y * self.w + x] = Some((style, g.into()));
                    }
                    x += 1;
                    for _ in 0..(width - 1) {
                        if x < self.w {
                            buf[y * self.w + x] = None;
                        }
                        x += 1;
                    }
                }
            }
        }
    }
}
