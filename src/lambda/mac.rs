macro_rules! check_both {
    (
        $expr:ty,
        $expect:ty $(,)?
    ) => {
        const _: () = {
            #[allow(unused)]
            type __Expr = $expr;
            #[allow(unused)]
            type __Expect = $expect;

            // TODO: Only opaquify expected
            check_type_eq!(Eval<__Expr>, Eval<__Expect>);
            check_type_eq!(Simp<__Expr>, Simp<__Expect>);
        };
    };
}
pub(crate) use check_both;

macro_rules! check_both_in {
    (
        $wrap:ident,
        $expr:ty,
        $expect:ty $(,)?
    ) => {
        check_both! { $wrap<$expr>, $wrap<$expect> }
    };
}
pub(crate) use check_both_in;
