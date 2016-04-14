// http://rosettacode.org/wiki/Define_a_primitive_data_type
// Implements a custom type named CustomInt.
// This type only implements a subset of all traits within std::ops.
use std::ops;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
struct CustomInt {
    value: u8,
}

enum CustomIntError {
    OutOfBoundsAssn,
}

impl CustomInt {
    fn custom_int(v: u8) -> Result<CustomInt, CustomIntError> {
        if v < 1 || v > 10 {
            Err(CustomIntError::OutOfBoundsAssn)
        } else {
            Ok(CustomInt { value: v })
        }
    }
}
// custom trait to specify bounds
trait Bounded {
    fn in_bounds(&self);
}

impl Bounded for CustomInt {
    fn in_bounds(&self) {
        if self.value < 1 || self.value > 10 {
            panic(self.value);
        }
        #[cold]
        #[inline(never)]
        fn panic(v: u8) -> ! {
            panic!("CustomInt is out of bounds! {} was value", v);
        }
    }
}

impl ops::Add for CustomInt {
    type Output = CustomInt;
    fn add(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value + rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::Sub for CustomInt {
    type Output = CustomInt;
    fn sub(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value - rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::Mul for CustomInt {
    type Output = CustomInt;
    fn mul(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value * rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::Div for CustomInt {
    type Output = CustomInt;
    fn div(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value / rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::BitAnd for CustomInt {
    type Output = CustomInt;
    fn bitand(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value & rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::BitOr for CustomInt {
    type Output = CustomInt;
    fn bitor(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value | rhs.value) };
        rval.in_bounds();
        rval
    }
}

impl ops::BitXor for CustomInt {
    type Output = CustomInt;
    fn bitxor(self, rhs: CustomInt) -> CustomInt {
        let rval = CustomInt { value: (self.value ^ rhs.value) };
        rval.in_bounds();
        rval
    }
}

fn main() {
    let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
    let cint_3: CustomInt = CustomInt::custom_int(3).ok().unwrap();
    let cint_4: CustomInt = CustomInt::custom_int(4).ok().unwrap();
    cint_2 + cint_4; // CustomInt { value: 6 }
    cint_4 - cint_2; // CustomInt { value: 2 }
    cint_4 * cint_2; // CustomInt { value: 8 }
    cint_4 / cint_2; // CustomInt { value: 2 }
    cint_3 & cint_2; // CustomInt { value: 2 }
    cint_3 | cint_2; // CustomInt { value: 3 }
    cint_3 ^ cint_2; // CustomInt { value: 1 }
}

#[cfg(test)]
mod tests {
    use super::CustomInt;

    #[test]
    fn add_test() {
        let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
        let cint_4: CustomInt = CustomInt::custom_int(4).ok().unwrap();
        assert_eq!(CustomInt::custom_int(6).ok().unwrap(), cint_2 + cint_4);
    }

    #[test]
    fn sub_test() {
        let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
        let cint_4: CustomInt = CustomInt::custom_int(4).ok().unwrap();
        assert_eq!(CustomInt::custom_int(2).ok().unwrap(), cint_4 - cint_2);
    }

    #[test]
    fn mul_test() {
        let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
        let cint_4: CustomInt = CustomInt::custom_int(4).ok().unwrap();
        assert_eq!(CustomInt::custom_int(8).ok().unwrap(), cint_4 * cint_2);
    }

    #[test]
    fn div_test() {
        let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
        let cint_4: CustomInt = CustomInt::custom_int(4).ok().unwrap();
        assert_eq!(CustomInt::custom_int(2).ok().unwrap(), cint_4 / cint_2);
    }

    #[test]
    fn and_test() {
        let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
        let cint_3: CustomInt = CustomInt::custom_int(3).ok().unwrap();
        assert_eq!(CustomInt::custom_int(2).ok().unwrap(), cint_3 & cint_2);
    }

    #[test]
    fn or_test() {
        let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
        let cint_3: CustomInt = CustomInt::custom_int(3).ok().unwrap();
        assert_eq!(CustomInt::custom_int(3).ok().unwrap(), cint_3 | cint_2);
    }

    #[test]
    fn xor_test() {
        let cint_2: CustomInt = CustomInt::custom_int(2).ok().unwrap();
        let cint_3: CustomInt = CustomInt::custom_int(3).ok().unwrap();
        assert_eq!(CustomInt::custom_int(1).ok().unwrap(), cint_3 ^ cint_2);
    }

    #[test]
    fn assn_out_of_bounds_test() {
        let cint_error = CustomInt::custom_int(0);
        assert!(cint_error.is_err());
    }

    #[test]
    #[should_panic(expected = "CustomInt is out of bounds! 11 was value")]
    fn above_out_of_bounds_test() {
        let cint_10: CustomInt = CustomInt::custom_int(10).ok().unwrap();
        let cint_1: CustomInt = CustomInt::custom_int(1).ok().unwrap();
        cint_10 + cint_1; // should panic here
    }

    #[test]
    #[should_panic(expected = "CustomInt is out of bounds! 0 was value")]
    fn below_out_of_bounds_test() {
        let cint_1: CustomInt = CustomInt::custom_int(1).ok().unwrap();
        cint_1 - cint_1; // should panic here
    }
}
