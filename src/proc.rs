crate::utils::mk_nat!({
    trait Nat {}

    trait Bit {
        type IfVal<T: LazyVal, F: LazyVal>: Value = If<Self, T::Eval, F::Eval>;
        type IfFlow<T: LazyFlow, F: LazyFlow>: Flow = If<Self, T::Eval, F::Eval>;
        type IfEnv<T: LazyEnv, F: LazyEnv>: Env = If<Self, T::Eval, F::Eval>;
        type IfVars<T: LazyVarList, F: LazyVarList>: VarList = If<Self, T::Eval, F::Eval>;
        type IfExpr<T: AstNode, F: AstNode>: AstNode = If<Self, T, F>;
    }
});

pub trait LazyVarList {
    type Eval: VarList;
}
impl<T: VarList> LazyVarList for T {
    type Eval = Self;
}
pub trait VarList {
    type Get<VarId: Nat>: Flow;
    type Set<VarId: Nat, Val: Value>: VarList;
}
impl VarList for () {
    type Get<VarId: Nat> = FlowThrow<Error<errors::UnknownVar<VarId>>>;
    type Set<VarId: Nat, Val: Value> = (VarId, Val, Self);
}
const _: () = {
    pub struct VarListGet<Env, Var>(Env, Var);
    impl<E: VarList, Var: Nat> LazyFlow for VarListGet<E, Var> {
        type Eval = E::Get<Var>;
    }
    pub struct EnvSetImplRec<Var, Val, Rest, Target, New>(Var, Val, Rest, Target, New);
    impl<Var: Nat, Val: Value, Rest: VarList, Target: Nat, New: Value> LazyVarList
        for EnvSetImplRec<Var, Val, Rest, Target, New>
    {
        type Eval = (Var, Val, Rest::Set<Target, New>);
    }
    impl<Var: Nat, Val: Value, Rest: VarList> VarList for (Var, Val, Rest) {
        type Get<Target: Nat> =
            <Target::Eq<Var> as Bit>::IfFlow<FlowVal<Val>, VarListGet<Rest, Target>>;
        type Set<Target: Nat, New: Value> = <Target::Eq<Var> as Bit>::IfVars<
            (Var, New, Rest),
            EnvSetImplRec<Var, Val, Rest, Target, New>,
        >;
    }
};

pub trait LazyEnv {
    type Eval: Env;
}
impl<E: Env> LazyEnv for E {
    type Eval = E;
}
// FIXME: support return and co
pub trait Env {
    type Flow: Flow;
    type Vars: VarList;

    type PropStoreIn<Var: Nat>: Env;
    type PropThen<Next: AstNode>: Env;
    type PropIfCond<Then: AstNode, Else: AstNode>: Env;
    type PropCall<Arg: AstNode>: Env;
    type PropPassTo<Func: Value>: Env;
    type PropAnd<R: AstNode>: Env;
    type PropOr<R: AstNode>: Env;
}
pub struct Environ<Vars, Tail>(Vars, Tail);
impl<Vars: VarList, Tail: Flow> Env for Environ<Vars, Tail> {
    type Flow = Tail;
    type Vars = Vars;

    type PropStoreIn<Var: Nat> = Tail::PropStoreIn<Vars, Var>;
    type PropThen<Next: AstNode> = Tail::PropThen<Vars, Next>;
    type PropIfCond<Then: AstNode, Else: AstNode> = Tail::PropCond<Vars, Then, Else>;
    type PropCall<Arg: AstNode> = Tail::PropCall<Vars, Arg>;
    type PropPassTo<Func: Value> = Tail::PropPassTo<Vars, Func>;
    type PropAnd<R: AstNode> = Tail::PropAnd<Vars, R>;
    type PropOr<R: AstNode> = Tail::PropOr<Vars, R>;
}
pub trait LazyVal {
    type Eval: Value;
}
impl<N: Value> LazyVal for N {
    type Eval = N;
}
pub trait Value {
    type Pop: Flow;
    type Last: Flow;
    type Push<V: Value>: Flow;
    type Truthy: Bit;
    type Call<Arg: Value>: Flow;
}
macro_rules! val_def {
    (@Truthy) => {
        type Truthy = nat!(1);
    };
    (@NoCall) => {
        type Call<__V: Value> = FlowThrow<Error<errors::NotCallable<Self>>>;
    };
    (@NoList) => {
        type Pop = FlowThrow<Error<errors::NotListLike<Self>>>;
        type Push<__V: Value> = FlowThrow<Error<errors::NotListLike<Self>>>;
        type Last = FlowThrow<Error<errors::NotListLike<Self>>>;
    };
    ($($item:ident)*) => {
        $(val_def!(@ $item);)*
    };
}

pub trait LazyFlow {
    type Eval: Flow;
}
impl<N: Flow> LazyFlow for N {
    type Eval = N;
}
pub trait Flow {
    type PropStoreIn<Vars: VarList, Var: Nat>: Env;
    type PropThen<Vars: VarList, Next: AstNode>: Env;
    type PropCond<Vars: VarList, Then: AstNode, Else: AstNode>: Env;
    type PropCall<Vars: VarList, Arg: AstNode>: Env;
    type PropPassTo<Vars: VarList, Func: Value>: Env;
    type PropAnd<Vars: VarList, Rhs: AstNode>: Env;
    type PropOr<Vars: VarList, Rhs: AstNode>: Env;
}
pub struct FlowThrow<V>(V);
impl<V: Value> Flow for FlowThrow<V> {
    type PropStoreIn<Vars: VarList, Var: Nat> = Environ<Vars, Self>;
    type PropThen<Vars: VarList, Next: AstNode> = Environ<Vars, Self>;
    type PropCond<Vars: VarList, Then: AstNode, Else: AstNode> = Environ<Vars, Self>;
    type PropCall<Vars: VarList, Arg: AstNode> = Environ<Vars, Self>;
    type PropPassTo<Vars: VarList, Func: Value> = Environ<Vars, Self>;
    type PropAnd<Vars: VarList, Rhs: AstNode> = Environ<Vars, Self>;
    type PropOr<Vars: VarList, Rhs: AstNode> = Environ<Vars, Self>;
}
pub struct FlowVal<V>(V);
impl<V: Value> Flow for FlowVal<V> {
    type PropStoreIn<Vars: VarList, Var: Nat> = Environ<Vars::Set<Var, V>, Self>;
    type PropThen<Vars: VarList, Next: AstNode> = Next::Eval<Vars>;
    type PropCond<Vars: VarList, Then: AstNode, Else: AstNode> =
        <<V::Truthy as Bit>::IfExpr<Then, Else> as AstNode>::Eval<Vars>;
    type PropCall<Vars: VarList, Arg: AstNode> = <Arg::Eval<Vars> as Env>::PropPassTo<V>;
    type PropPassTo<Vars: VarList, Func: Value> = Environ<Vars, Func::Call<V>>;
    type PropAnd<Vars: VarList, Rhs: AstNode> =
        <<V::Truthy as Bit>::IfExpr<Rhs, Const<V>> as AstNode>::Eval<Vars>;
    type PropOr<Vars: VarList, Rhs: AstNode> =
        <<V::Truthy as Bit>::IfExpr<Const<V>, Rhs> as AstNode>::Eval<Vars>;
}

pub struct Error<Hint>(Hint);
pub mod errors {
    pub struct NotCallable<V>(V);
    pub struct NotListLike<V>(V);
    pub struct UnknownVar<V>(V);
    pub struct ListEmpty;
}
impl<H> Value for Error<H> {
    val_def!(NoList NoCall Truthy);
}

pub struct ValNat<N>(N);
impl<N: Nat> Value for ValNat<N> {
    type Pop = FlowVal<ValNat<N::Pop>>;
    type Push<V: Value> = FlowVal<ValNat<N::PushBit<V::Truthy>>>;
    type Last = FlowVal<ValNat<N::Par>>;
    type Truthy = N::IsNonZero;

    val_def!(NoCall);
}
pub trait List {
    type Pop: Flow;
    type Last: Flow;
    type NonEmpty: Bit;
}
impl List for () {
    type Pop = FlowThrow<Error<errors::ListEmpty>>;
    type Last = FlowThrow<Error<errors::ListEmpty>>;
    type NonEmpty = nat!(0);
}
impl<Pop: List, Last: Value> List for (Pop, Last) {
    type Pop = FlowVal<ValList<Pop>>;
    type Last = FlowVal<Last>;
    type NonEmpty = nat!(0);
}
pub struct ValList<L>(L);
impl<L: List> Value for ValList<L> {
    type Pop = L::Pop;
    type Last = L::Last;
    type Push<V: Value> = FlowVal<ValList<(L, V)>>;
    type Truthy = L::NonEmpty;
    type Call<Arg: Value> = FlowThrow<Error<errors::NotCallable<Self>>>;
}
pub type EmptyList = Const<ValList<()>>;

pub struct NilVal;
impl Value for NilVal {
    type Truthy = nat!(0);

    val_def!(NoList NoCall);
}
pub type Nil = Const<NilVal>;

pub trait AstNode {
    type Eval<E: VarList>: Env;
}
pub type EvalEnv<S, Vars = ()> = <S as AstNode>::Eval<Vars>;
pub type Eval<S, Vars = ()> = <EvalEnv<S, Vars> as Env>::Flow;

impl<A: AstNode, B: AstNode> AstNode for (A, B) {
    type Eval<E: VarList> = <A::Eval<E> as Env>::PropThen<B>;
}

pub struct VarSet<Var: Nat, Rhs: AstNode>(Var, Rhs);
impl<V: Nat, R: AstNode> AstNode for VarSet<V, R> {
    type Eval<E: VarList> = <R::Eval<E> as Env>::PropStoreIn<V>;
}
pub struct Var<Var: Nat>(Var);
impl<V: Nat> AstNode for Var<V> {
    type Eval<E: VarList> = Environ<E, E::Get<V>>;
}

pub struct Const<Val: Value>(Val);
impl<Val: Value> AstNode for Const<Val> {
    type Eval<E: VarList> = Environ<E, FlowVal<Val>>;
}
pub struct If<Cond: AstNode, Then: AstNode, Else: AstNode = Const<NilVal>>(Cond, Then, Else);
impl<Cond: AstNode, Then: AstNode, Else: AstNode> AstNode for If<Cond, Then, Else> {
    type Eval<E: VarList> = <Cond::Eval<E> as Env>::PropIfCond<Then, Else>;
}
pub struct While<Cond: AstNode, Block: AstNode, Else: AstNode = Const<NilVal>>(Cond, Block, Else);
impl<Cond: AstNode, Block: AstNode, Else: AstNode> AstNode for While<Cond, Block, Else> {
    type Eval<E: VarList> = <If<Cond, (Block, Self), Else> as AstNode>::Eval<E>;
}

pub type Push = Const<_PushFunc>;
pub struct _PushFunc;
const _: () = {
    impl Value for _PushFunc {
        type Call<Arg: Value> = FlowVal<_PushFuncPartial<Arg>>;
        val_def!(NoList Truthy);
    }
    pub struct _PushFuncPartial<L>(L);
    impl<L: Value> Value for _PushFuncPartial<L> {
        type Call<Arg: Value> = L::Push<Arg>;
        val_def!(NoList Truthy);
    }
};
pub type Pop = Const<_PopFunc>;
pub struct _PopFunc;
impl Value for _PopFunc {
    type Call<Arg: Value> = Arg::Pop;
    val_def!(NoList Truthy);
}
pub type Last = Const<_LastFunc>;
pub struct _LastFunc;
impl Value for _LastFunc {
    type Call<Arg: Value> = Arg::Last;
    val_def!(NoList Truthy);
}

pub struct Lambda<Param: Nat, Tail: AstNode>(Param, Tail);
const _: () = {
    impl<P: Nat, T: AstNode> AstNode for Lambda<P, T> {
        type Eval<E: VarList> = Environ<E, FlowVal<Func<E, P, T>>>;
    }

    pub struct Func<Envi: VarList, Param: Nat, Tail: AstNode>(Envi, Param, Tail);
    impl<E: VarList, P: Nat, T: AstNode> Value for Func<E, P, T> {
        type Call<Arg: Value> = <T::Eval<(P, Arg, E)> as Env>::Flow;

        val_def!(NoList Truthy);
    }
};
pub struct Call<Func: AstNode, Arg: AstNode>(Func, Arg);
impl<F: AstNode, A: AstNode> AstNode for Call<F, A> {
    type Eval<E: VarList> = <F::Eval<E> as Env>::PropCall<A>;
}

pub struct And<L: AstNode, R: AstNode>(L, R);
impl<L: AstNode, R: AstNode> AstNode for And<L, R> {
    type Eval<E: VarList> = <L::Eval<E> as Env>::PropAnd<R>;
}
pub struct Or<L: AstNode, R: AstNode>(L, R);
impl<L: AstNode, R: AstNode> AstNode for Or<L, R> {
    type Eval<E: VarList> = <L::Eval<E> as Env>::PropOr<R>;
}

// Note that functions capture their environment when they are created.
// As such, simply storing a function in a variable does not provide
// recursion. This helper module provides recursion using the T combinator.
mod recursion {
    use super::*;
    type F = nat!(0);
    type X = nat!(1);
    type V = nat!(2);
    type ZInner = Lambda<
        X, //
        Call<
            Var<F>, //
            Lambda<V, Call<Call<Var<X>, Var<X>>, Var<V>>>,
        >,
    >;
    pub type ZComb = Lambda<F, Call<ZInner, ZInner>>;

    // Automatically sticks a function expression into the Z combinator.
    // The first argument should be some variable id, which will be
    // available in the context of the function, and contain the function
    // itself.
    pub type Recursive<This, Func> = Call<ZComb, Lambda<This, Func>>;
}
pub use recursion::*;

pub trait VarGetVar {
    type Var: Nat;
}
impl<V: Nat> VarGetVar for V {
    type Var = V;
}
impl<V: Nat> VarGetVar for Var<V> {
    type Var = V;
}

#[allow(unused_macros)]
macro_rules! vars {
    ($($v:ident = $id:literal),* $(,)?) => {
        $( type $v = Var<nat!($id)>; )*
    };
}

#[allow(unused_macros)]
macro_rules! expr {
    ([$t:ty]) => { $t };

    (($($expr:tt)*)) => { expr!($($expr)*) };
    ($var_or_name:ident) => { $var_or_name };
    ($literal:literal) => { Const<ValNat<nat!($literal)>> };

    (fn $param:ident $($rparam:ident)+ => $($body:tt)+) => {
        Lambda<<$param as VarGetVar>::Var, expr!(fn $($rparam)* => $($body)*)>
    };
    (fn $param:ident => $($body:tt)+) => {
        Lambda<<$param as VarGetVar>::Var, expr!($($body)*)>
    };
    (use $rec:ident; $($expr:tt)*) => {
        Recursive<<$rec as VarGetVar>::Var, expr!($($expr)*)>
    };
    (if $cond:tt { $($then:tt)* } $(else { $($else:tt)* })?) => {
        If<expr!($cond), expr!($($then)*), $(expr!($($else)*))?>
    };
    (while $cond:tt { $($then:tt)* } $(else { $($else:tt)* })?) => {
        While<expr!($cond), expr!($($then)*), $(expr!($($else)*))?>
    };
    (type $alias:ident $($expr:tt)*) => {
        $alias<$(expr!($expr)),*>
    };

    ($lhs:ident = $($rhs:tt)+) => {
        VarSet<<$lhs as VarGetVar>::Var, expr!($($rhs)*)>
    };

    // enforce that this is parenthesized to not accidentally
    // parse `A = B; C` as `A = (B; C)`
    (($($first:tt)*); $($then:tt)*) => {
        (expr!($($first)*), expr!($($then)*))
    };

    ($mac:ident ! $($rest:tt)*) => {
        $mac! { $($rest)* }
    };

    ($func:tt $arg:tt $($rest:tt)*) => {
        expr!([Call<expr!($func), expr!($arg)>] $($rest)*)
    };
}

#[allow(unused_macros)]
macro_rules! toks {
    ($($t:tt)*) => {
        $($t)*
    };
}

#[cfg(test)]
toks! {
    pub mod logic;
    pub use logic::*;
    pub mod list;
    pub use list::*;
    pub mod add;
    pub use add::*;
    pub mod cmp;
    pub use cmp::*;
    pub mod sub;
    pub use sub::*;
}
