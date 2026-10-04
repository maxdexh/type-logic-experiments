use macro_rules_attribute::apply;

pub trait Node: NodeExpr<Eval = Self> {
    type Item: Node;
    type Next: Node;
    type If<T: NodeExpr, F: NodeExpr>: Node;
}
pub trait NodeExpr {
    type Eval: Node;
}
impl NodeExpr for () {
    type Eval = Self;
}
impl Node for () {
    type Item = Self;
    type Next = Self;
    type If<T: NodeExpr, F: NodeExpr> = F::Eval;
}
impl<I: Node, N: Node> NodeExpr for (I, N) {
    type Eval = Self;
}
impl<I: Node, N: Node> Node for (I, N) {
    type Item = I;
    type Next = N;
    type If<T: NodeExpr, F: NodeExpr> = T::Eval;
}

pub type Eval<X> = <X as NodeExpr>::Eval;

macro_rules! node_expr {
    ($v:vis type $name:ident<$($gp:ident),*> = $val:ty;) => {
        $v struct $name<$($gp),*>($($gp),*);
        impl<$($gp: NodeExpr),*> NodeExpr for $name<$($gp),*> {
            type Eval = Eval<$val>;
        }
    };
}

pub type Item<X> = <Eval<X> as Node>::Item;
pub type Next<X> = <Eval<X> as Node>::Next;

#[apply(node_expr)]
pub type If<C, T, F> = <Eval<C> as Node>::If<T, F>;
pub type Or<L, R> = If<L, L, R>;
pub type And<L, R> = If<L, R, L>;
pub type Not<X> = If<X, (), ((), ())>;
pub type Bool<X> = If<X, ((), ()), ()>;
