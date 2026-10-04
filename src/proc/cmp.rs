use super::*;

vars!(L = 0, R = 1, This = 2, CmpHalf = 3);

// taken from gnat
// 0 is Lt, 1 is Gt, 2 is Eq
pub type Cmp = expr! {
    use This;
    fn L R =>
    if L {
        if R {
            (CmpHalf = This (Pop L) (Pop R));

            if (Pop CmpHalf) {
                if (Last L) {
                    if (Last R) { 2 } else { 1 }
                } else {
                    if (Last R) { 0 } else { 2 }
                }
            } else {
                CmpHalf
            }
        } else {
            1
        }
    } else {
        if R { 0 } else { 2 }
    }
};

pub type Eq = expr! { fn L R => Pop (Cmp L R) };
pub type Ne = expr! { fn L R => Not (Eq L R) };
pub type Lt = expr! { fn L R => Not (Cmp L R) };
pub type Gt = expr! { fn L R => Lt R L };
pub type Le = expr! { fn L R => Not (Lt R L) };
pub type Ge = expr! { fn L R => Le R L };
