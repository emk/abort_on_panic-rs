#![feature(phase)]
#[phase(plugin, link)] extern crate abort_on_panic;

#[test]
pub fn test_macro() {
    let result = abort_on_panic!({ "value" });
    assert_eq!("value", result);

    abort_on_panic!("cannot panic inside FFI callbacks", {
        // ...
    });
}

