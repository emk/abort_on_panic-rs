//! When calling Rust code from C, it's unsafe to call `panic!`.  Doing so
//! may cause unsafe behavior.  But when calling user-defined functions,
//! we sometimes need to enforce these rules.
//!
//! ```ignore
//! #[macro_use]
//! extern crate abort_on_panic;
//! 
//! #[test]
//! pub fn test_macro() {
//!     let result = abort_on_panic!({ "value" });
//!     assert_eq!("value", result);
//! 
//!     abort_on_panic!("cannot panic inside FFI callbacks", {
//!         // ...
//!     });
//! }
//! ```

#![feature(core)]

use std::intrinsics::abort;
use std::io::{stderr, Write};
use std::thread::panicking;

/// Once this object is created, it can only be destroyed in an orderly
/// fashion.  Attempting to clean it up from a panic handler will abort the
/// process.
pub struct PanicGuard {
    // We hope that this will be optimized heavily.
    message: Option<&'static str>
}

impl PanicGuard {
    /// Create a panic guard with a generic message.
    pub fn new() -> PanicGuard { PanicGuard{message: None} }

    /// Create a panic guard with a custom message.
    pub fn with_message(message: &'static str) -> PanicGuard {
        PanicGuard{message: Some(message)}
    }
}

impl Drop for PanicGuard {
    fn drop(&mut self) {
        // At the suggestion of Daniel Grunwald, check that we actually
        // have a task before calling `failing()`.  If we have no task to
        // catch this panic, `failing()` will always panic, even on
        // success.  But in this case, the runtime will also abort on panic
        // automatically, so we can just do nothing.
        if panicking() {
            let msg = self.message.unwrap_or("cannot unwind past stack frame");
            let _ = writeln!(&mut stderr(), "{} at {}:{}",
                             msg, file!(), line!());
            unsafe { abort(); }
        }
    }
}

/// Run a block of code, aborting the entire process if it tries to panic.
#[macro_export]
macro_rules! abort_on_panic {
    ($message:expr, $body:block) => {
        {
            let guard = ::abort_on_panic::PanicGuard::with_message($message);
            let result = $body;
            drop(guard);
            result
        }
    };

    ($body:block) => {
        {
            let guard = ::abort_on_panic::PanicGuard::new();
            let result = $body;
            drop(guard);
            result
        }
    };
}
