# cursive_buffered_backend

[![crates.io](https://meritbadge.herokuapp.com/cursive_buffered_backend)](https://crates.io/crates/cursive_buffered_backend)
[![Build Status](https://travis-ci.com/agavrilov/cursive_buffered_backend.svg?branch=master)](https://travis-ci.com/agavrilov/cursive_buffered_backend)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

The buffering backend for any [Cursive](https://github.com/gyscos/Cursive) backend. Mainly it is created to address a [flickering issue](https://github.com/gyscos/Cursive/issues/142) with Termion backend.

Inspired by the [comment](https://gitlab.redox-os.org/redox-os/termion/issues/105#note_6769) on the similar issue on Termion itself.

# Usage

```rust
let mut app = Cursive::new(|| {
    let termion_backend = backend::termion::Backend::init();
    let buffered_backend = cursive_buffered_backend::BufferedBackend::new(termion_backend);
    Box::new(buffered_backend)
});

```
