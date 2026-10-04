use super::*;

vars!(A = 0, This = 1);

pub type PredUnchecked = expr! {
    use This; fn A =>

    if (Last A) {
        Push (Pop A) 0
    } else {
        Push (This (Pop A)) 1
    }
};
pub type Pred = expr! {
    fn A => if A {
        PredUnchecked A
    } else {
        0
    }
};
