[![Build Status](https://travis-ci.org/emk/abort_on_panic-rs.svg)](https://travis-ci.org/emk/abort_on_panic-rs)

[Documentation](http://www.rust-ci.org/emk/abort_on_panic-rs/doc/abort_on_panic).
[StackOverflow question](http://stackoverflow.com/questions/27384824/catching-panic-when-rust-called-from-c-ffi-without-spawning-threads) looking for alternatives.

When calling Rust code from C, it's unsafe to call `panic!`.  Doing so may
cause unsafe behavior.  But when calling user-defined functions, we
sometimes need to enforce these rules.

To use this library, add the following to your `Cargo.toml` file:

```
[dependencies]
abort_on_panic = "*"
```

You can than capture a `panic!` as follows:

```rust
#![feature(phase)]
#[phase(plugin, link)] extern crate abort_on_panic;

pub fn main() {
    let result = abort_on_panic!({
        "value"
    });
    assert_eq!("value", result);

    abort_on_panic!({
        panic!("Uh oh.");
    });
}
```

### Credits & license

The original idea for this code came from Benjamin Herr.  Many thanks!

This code is placed into the public domain under the terms described by the
Unlicense.
