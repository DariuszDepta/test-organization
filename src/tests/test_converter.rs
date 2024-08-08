use crate::convert_10;

#[test]
fn converting_with_clamp_10_should_work() {
    assert_eq!(-10, convert_10(-11, 1, 0));
    assert_eq!(-10, convert_10(-10, 1, 0));
    assert_eq!(-8, convert_10(-8, 1, 0));
    assert_eq!(0, convert_10(0, 1, 0));
    assert_eq!(8, convert_10(8, 1, 0));
    assert_eq!(10, convert_10(10, 1, 0));
    assert_eq!(10, convert_10(11, 1, 0));
}
