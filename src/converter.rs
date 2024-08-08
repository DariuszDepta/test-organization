const C1: i64 = 10;
const C2: i64 = 20;
const C3: i64 = 30;

pub fn convert_10(n: i64, scale: i64, offset: i64) -> i64 {
    clamp_10(n).saturating_mul(scale).saturating_add(offset)
}

pub fn convert_20(n: i64, scale: i64, offset: i64) -> i64 {
    clamp_20(n).saturating_mul(scale).saturating_add(offset)
}

pub fn convert_30(n: i64, scale: i64, offset: i64) -> i64 {
    clamp_30(n).saturating_mul(scale).saturating_add(offset)
}

fn clamp_10(n: i64) -> i64 {
    clamp(n, C1)
}

fn clamp_20(n: i64) -> i64 {
    clamp(n, C2)
}

fn clamp_30(n: i64) -> i64 {
    clamp(n, C3)
}

fn clamp(n: i64, c: i64) -> i64 {
    n.clamp(-c, c)
}

// (1)
#[test]
fn clamping_should_work() {
    use crate::generator::get_number;

    // testing private functions of the same module
    assert_eq!(-10, clamp(-11, 10));
    // testing public functions of the same module
    assert_eq!(-10, convert_10(-11, 1, 0));
    // testing public function of another module or public function of the library
    assert_eq!(-57, get_number(-31));
}

// (2)
#[cfg(test)]
mod tests {
    use super::{clamp, convert_10};
    use crate::generator::get_number;

    #[test]
    fn clamping_should_work() {
        // testing private functions of the parent module
        assert_eq!(3, clamp(4, 3));
        // testing public functions of the parent module
        assert_eq!(-10, convert_10(-11, 1, 0));
        // testing public function of another module or public function of the library
        assert_eq!(-57, get_number(-31));
    }
}
