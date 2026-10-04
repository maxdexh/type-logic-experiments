crate::utils::mk_nat!({
    trait Nat {}

    trait Bit {
        type ThenExitGet<Sym: Nat, V: Val, Rest: Env>: Val = If<
            Self, //
            V,
            Rest::Get<Sym>,
        >;
        type IfExpr<T: Expr, F: Expr>: Expr = If<Self, T, F>;
        type ThenSubs<X: Expr, Sym: Nat, Sub: Expr>: Expr = If<
            Self, //
            X::Subs<Sym, Sub>,
            X,
        >;
    }
});

/// A value in the semantics, i.e. a function
pub trait Val {
    type Apply<Arg: Val>: Val;
}
/// A syntactic expression in lambda calculus
pub trait Expr {
    // Value-based computation
    type Eval<E: Env>: Val;

    // Symbolic computation
    type Simp: SimpExpr;
    type Subs<Sym: Nat, Sub: Expr>: Expr;
}
// a fully simplified expr
pub trait SimpExpr: Expr {
    type SimpJux<Arg: SimpExpr>: SimpExpr;
}

/// The environment an expression is evaluated in
pub trait Env {
    type Get<Sym: Nat>: Val;
}

pub struct Error<Hint>(Hint);
impl<H> Val for Error<H> {
    type Apply<Arg: Val> = Self;
}
impl<H> Expr for Error<H> {
    type Eval<E: Env> = Self;
    type Subs<Sym: Nat, Sub: Expr> = Self;
    type Simp = Self;
}
impl<H> SimpExpr for Error<H> {
    type SimpJux<Arg: SimpExpr> = Self;
}

// Opaque symbol/value to use in tests
pub struct Opq<H>(H);
impl<H> Expr for Opq<H> {
    type Eval<E: Env> = Self;
    type Simp = Self;
    type Subs<Sym: Nat, Sub: Expr> = Self;
}
impl<H> SimpExpr for Opq<H> {
    type SimpJux<Arg: SimpExpr> = OpqJux<Self, Arg>;
}
impl<H> Val for Opq<H> {
    type Apply<Arg: Val> = OpqJux<Self, Arg>;
}

// Opaque juxtaposition in tests (for juxtaposition with opaque lhs)
pub struct OpqJux<L, R>(L, R);
impl<L: SimpExpr, R: SimpExpr> Expr for OpqJux<L, R> {
    type Subs<Sym: Nat, Sub: Expr> = Jux<L::Subs<Sym, Sub>, R::Subs<Sym, Sub>>;
    type Eval<E: Env> = <Jux<L, R> as Expr>::Eval<E>;
    type Simp = Self; // = <Jux<L, R> as Expr>::Simp // L, R already fully simplified
}
impl<L: SimpExpr, R: SimpExpr> SimpExpr for OpqJux<L, R> {
    type SimpJux<Arg: SimpExpr> = OpqJux<Self, Arg>;
}
impl<L: Val, R: Val> Val for OpqJux<L, R> {
    type Apply<Arg: Val> = OpqJux<Self, Arg>; // L is opaque, so know nothing about what it returns
}

pub struct FuncVal<Env, Param, Body>(Env, Param, Body);
impl<E: Env, P: Nat, B: Expr> Val for FuncVal<E, P, B> {
    type Apply<Arg: Val> = B::Eval<(P, Arg, E)>;
}
impl Env for () {
    type Get<Sym: Nat> = Error<Sym>;
}
impl<K: Nat, V: Val, Rest: Env> Env for (K, V, Rest) {
    type Get<Sym: Nat> = <Sym::Eq<K> as Bit>::ThenExitGet<Sym, V, Rest>;
}

pub struct Jux<L, R>(L, R);
impl<L: Expr, R: Expr> Expr for Jux<L, R> {
    type Eval<E: Env> = <L::Eval<E> as Val>::Apply<R::Eval<E>>;

    type Simp = <L::Simp as SimpExpr>::SimpJux<R::Simp>;
    type Subs<Sym: Nat, Sub: Expr> = Jux<L::Subs<Sym, Sub>, R::Subs<Sym, Sub>>;
}
pub struct Lam<Param, Body>(Param, Body);
impl<P: Nat, B: Expr> Expr for Lam<P, B> {
    type Eval<E: Env> = FuncVal<E, P, B>;

    type Simp = Self;
    type Subs<Sym: Nat, Sub: Expr> =
        Lam<P, <<P::Eq<Sym> as Bit>::Not as Bit>::ThenSubs<B, Sym, Sub>>;
}
impl<P: Nat, B: Expr> SimpExpr for Lam<P, B> {
    type SimpJux<Arg: SimpExpr> = <B::Subs<P, Arg> as Expr>::Simp;
}

pub struct Sym<S>(S);
impl<S: Nat> Expr for Sym<S> {
    type Eval<E: Env> = E::Get<S>;

    type Simp = Error<S>;
    type Subs<Target: Nat, Sub: Expr> = <S::Eq<Target> as Bit>::IfExpr<Sub, Self>;
}

pub type Eval<E> = <E as Expr>::Eval<()>;
pub type Simp<E> = <E as Expr>::Simp;

pub trait SymOrId {
    type Id: Nat;
}
impl<N: Nat> SymOrId for N {
    type Id = N;
}
impl<N: Nat> SymOrId for Sym<N> {
    type Id = N;
}

#[allow(unused_macros)]
macro_rules! expr {
    ( [$($t:tt)*] ) => {
        $($t)*
    };
    ($var:ident: $($body:tt)+) => {
        Lam<<$var as SymOrId>::Id, expr!($($body)*)>
    };
    ($func:tt $arg:tt $($rest:tt)*) => {
        expr!([Jux<expr!($func), expr!($arg)>] $($rest)*)
    };
    ($var:ident) => {
        $var
    };
    ( ($($t:tt)*) ) => {
        expr!($($t)*)
    };
    ($($rest:tt)*) => {
        compile_error!(concat!("Invalid syntax: `", stringify!($($rest)*), "`"))
    }
}

#[allow(unused_macros)]
macro_rules! toks {
    ($($t:tt)*) => {
        $($t)*
    };
}

//#[cfg(feature = "lambda-tests")]
#[cfg(test)]
toks! {
    use crate::utils::check_type_eq;

    mod symbols;
    use symbols::*;

    mod mac;
    use mac::*;

    pub mod bools;
    pub use bools::*;

    pub mod nats;
    pub use nats::*;

    pub mod nats2;
    pub use nats2::*;

    pub mod recursion;
    pub use recursion::*;
}
