#[macro_use]
extern crate abort_on_panic;

fn oops() {
    panic!("something went wrong!");
}

fn main() {
    abort_on_panic!("cannot panic inside this block", {
        oops();
    });
}
