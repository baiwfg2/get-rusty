mod common;

use get_rusty::add_two;

#[test]
fn dummy_test() {
    common::setup();
    assert_eq!(add_two(2, 2), 4);
}