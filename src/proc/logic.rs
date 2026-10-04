use super::*;

vars!(A = 0, B = 1);

pub type Not = expr! { fn A => if A { 0 } else { 1 } };
pub type ToBool = expr! { fn A => if A { 1 } else { 0 } };

pub type Xor = expr! {
    fn A B => if A {
        Not B
    } else {
        B
    }
};
