use super::*;

// A natural number N is represented as the function that takes
// F and X, and applies F to X a total of N times. i.e. it sends
// F to F^n, which is F composed with itself N times
pub type N0 = expr! { F: X: X };
pub type N1 = expr! { F: X: F X };
pub type N2 = expr! { F: X: F (F X) };

check_type_eq!(N2, Lam<IdOf<F>, Lam<IdOf<X>, Jux<F, Jux<F, X>>>>);

check_both!(expr!(N0 OF OX), expr!(OX));
check_both!(expr!(N1 OF OX), expr!(OF OX));
check_both!(expr!(N2 OF OX), expr!(OF (OF OX)));

// 0 and false are the same object
check_both_in!(Opq2, N0, False);
// 1 and the identity are the same object (modulo laziness)
check_both_in!(Opq2, N1, Identity);

// succ takes a number n and sends it to the function applying
// f to x a total of n + 1 times, i.e. the function representing n + 1
pub type Succ = expr! { N: F: X: F (N F X) };

check_both_in!(Opq2, expr!(Succ N0), N1);
check_both_in!(Opq2, expr!(Succ N1), N2);

pub type N3 = expr!(Succ N2);
pub type N4 = expr!(Succ N3);
pub type N5 = expr!(Succ N4);
pub type N6 = expr!(Succ N5);

// to check if a natural number N is zero, we use the definition.
// Every natural number except for 0 applies its first argument
// to the second at least once.
pub type IsZero = expr!(N: N (X: False) True);

// we can even prove correctness symbolically :D
check_both_in!(Opq2, expr!(IsZero N0), True);
check_both_in!(Opq2, expr!(IsZero (Succ ON)), False);
