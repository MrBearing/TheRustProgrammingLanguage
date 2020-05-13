extern crate showing_function_output;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, showing_function_output::adder::add_two(2));
}