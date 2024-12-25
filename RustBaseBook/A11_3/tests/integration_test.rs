mod common;

use A11_3::add_two;


#[test]
fn test_add() {
    assert_eq!(add_two(2), 4);
}
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(add_two(2), 4);
}