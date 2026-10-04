use super::*;

pub type X = Sym<nat!(1)>;
pub type Y = Sym<nat!(6)>;
pub type F = Sym<nat!(2)>;
pub type G = Sym<nat!(5)>;
pub type M = Sym<nat!(4)>;
pub type N = Sym<nat!(3)>;
pub type B = Sym<nat!(7)>;
pub type C = Sym<nat!(8)>;

pub type IdOf<S> = <S as SymOrId>::Id;

macro_rules! decl_opaques {
    ($modname:ident, [$($var:ident),*]) => {
        mod $modname {
            $(pub struct $var;)*
        }
        $(pub type $var = Opq<$modname::$var>;)*
    };
}
decl_opaques!(opaques, [OX, OY, OF, OG, ON, OM]);

pub type Opq2<N> = expr!(N OX OY);

// the identity function
pub type Identity = expr!(X: X);
// composition function
pub type Compose = expr!(F: G: X: F (G X));

check_type_eq!(
    Eval<expr!(F: F F)>,
    FuncVal<(), IdOf<F>, Jux<F, F>>,
);
check_type_eq!(
    Eval<Identity>,
    FuncVal<(), IdOf<X>, X>,
);
check_type_eq!(
    Eval<Compose>,
    FuncVal<(), IdOf<F>, Lam<IdOf<G>, Lam<IdOf<X>, Jux<F, Jux<G, X>>>>>,
);
check_type_eq!(Simp<Identity>, Identity);
check_type_eq!(Simp<Compose>, Compose);

check_type_eq!(Simp<expr!(Compose OF OG)>, expr!(X: OF (OG X)));
check_type_eq!(Simp<expr!(Identity OX)>, OX);
