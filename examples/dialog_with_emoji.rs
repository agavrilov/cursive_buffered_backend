use std::io::Error;

use cursive::{
    views::{CircularFocus, Dialog, TextView},
    Cursive, With as _,
};

fn main() -> Result<(), Error> {
    let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
        Ok(Box::new(buffered_backend))
        //Ok(backend)
    };

    // Creates the cursive root - required for every application.
    let mut siv = Cursive::new();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        // Most views can be configured in a chainable way
        Dialog::around(TextView::new("🖼️🖼️Preview"))
            .button("🖼️🖼️Get", |_s| ())
            .button("Exit", |s| s.quit())
            .wrap_with(CircularFocus::new)
            .wrap_tab(),
    );

    // Starts the event loop.
    siv.try_run_with(backend_init)
}
