#![feature(phase)]
#[phase(plugin, link)] extern crate abort_on_panic;

fn main() {
    abort_on_panic!("cannot panic inside this block", {
        panic!("something went wrong!");
    });
}
