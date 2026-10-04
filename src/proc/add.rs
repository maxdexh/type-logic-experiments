use super::*;

vars!(A = 0, B = 1, C = 2, This = 3);

pub type Succ = expr! {
    use This; fn A =>

    if (Last A) {
        Push (This (Pop A)) 0
    } else {
        Push (Pop A) 1
    }
};
type Xor3 = expr! { fn A B C => Xor A (Xor B C) };
type AtLeast2 = expr! {
    fn A B C => if A {
        if B { 1 } else { C }
    } else {
        if B { C } else { 0 }
    }
};
type CarryAdd = expr! {
    use This;
    fn C A B => if A {
        Push
        (
            This
            (AtLeast2 (Last A) (Last B) C)
            (Pop A)
            (Pop B)
        )
        (Xor3 (Last A) (Last B) C)
    } else {
        if C { Succ B } else { B }
    }
};
pub type Add = expr! { CarryAdd 0 };

mod tests {
    use super::*;
    use crate::utils::check_type_eq;

    type A = nat!(0);
    type B = nat!(1);
    check_type_eq!(
        Eval<Call<Lambda<B, Var<B>>, Const<ValNat<nat!(2)>>>>,
        FlowVal<ValNat<nat!(2)>>
    );
    check_type_eq!(
        Eval<Call<Lambda<B, Var<A>>, Const<ValNat<nat!(2)>>>>,
        FlowThrow<Error<errors::UnknownVar<A>>>,
    );
    check_type_eq!(Eval<Var<B>>, FlowThrow<Error<errors::UnknownVar<B>>>);

    check_type_eq!(Eval<expr!(Succ 1)>, FlowVal<ValNat<nat!(2)>>);
    check_type_eq!(Eval<expr!(Succ 2)>, FlowVal<ValNat<nat!(3)>>);

    check_type_eq!(Eval<expr!(Add 2 2)>, FlowVal<ValNat<nat!(4)>>);
}
