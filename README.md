# cursive_buffered_backend

[![crates.io](https://img.shields.io/crates/v/cursive_buffered_backend.svg)](https://crates.io/crates/cursive_buffered_backend)
[![Build Status](https://travis-ci.com/agavrilov/cursive_buffered_backend.svg?branch=master)](https://travis-ci.com/agavrilov/cursive_buffered_backend)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

The buffering backend for any [Cursive](https://github.com/gyscos/Cursive) backend. Mainly it is created to address a [flickering issue](https://github.com/gyscos/Cursive/issues/142) with Termion backend.

Inspired by the [comment](https://gitlab.redox-os.org/redox-os/termion/issues/105#note_6769) on the similar issue on Termion itself.

# Usage

```rust
let backend_init = || -> std::io::Result<Box<dyn cursive::backend::Backend>> {
    let backend = cursive::backends::termion::Backend::init()?;
    let buffered_backend = cursive_buffered_backend::BufferedBackend::new(backend);
    Ok(Box::new(buffered_backend))
};

let mut app = Cursive::new();
app.try_run_with(backend_init).ok()?;
```
