mod common;

use rust::add_two;

#[test]
fn dummy_test() {
    common::setup();
    assert_eq!(add_two(2, 2), 4);
}