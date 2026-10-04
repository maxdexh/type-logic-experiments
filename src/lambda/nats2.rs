use super::*;

// Add takes numbers m, n and sends it to the function applying
// f to x a total of m + n times, i.e. the function representing
// m + n.
pub type Add = expr!(M: N: F: X: M F (N F X));

// `Add M N F` is the composition of `M F` and `N F`, and we can
// prove this symbolically
check_both!(expr!(Add OM ON OF OX), expr!(Compose (OM OF) (ON OF) OX));

// we can even prove symbolically that N0 is a neutral element
check_both_in!(Opq2, expr!(Add N0 ON), ON);
check_both_in!(Opq2, expr!(Add ON N0), ON);
// and that 1 + n is the successor (but it only works on one side)
check_both_in!(Opq2, expr!(Add N1 ON), expr!(Succ ON));

// some inputs to add
check_both_in!(Opq2, expr!(Add N1 N2), N3);
check_both_in!(Opq2, expr!(Add N2 N1), N3);
check_both_in!(Opq2, expr!(Add N3 N1), N4);
check_both_in!(Opq2, expr!(Add N4 N1), N5);
check_both_in!(Opq2, expr!(Add N3 N2), N5);
check_both_in!(Opq2, expr!(Add N2 N3), N5);

// Alternative definition for addition: just compose the successor
// function with itself LHS times
pub type AddAlt = expr!(N: N Succ);
check_both_in!(Opq2, expr!(AddAlt N2 N2), N4);
check_both_in!(Opq2, expr!(AddAlt N2 N1), N3);
check_both_in!(Opq2, expr!(AddAlt N3 N2), N5);
check_both_in!(Opq2, expr!(AddAlt N4 N1), N5);

// Multiplication is easy: We need to apply F a total of M * N times.
// We can do this by applying N F a total of M times.
pub type Mul = expr!(M: N: F: M (N F));
check_both_in!(Opq2, expr!(Mul N2 N2), N4);
check_both_in!(Opq2, expr!(Mul N2 N3), N6);
check_both_in!(Opq2, expr!(Mul N0 N5), N0);
check_both_in!(Opq2, expr!(Mul N4 N1), N4);

// Nice theorems (only work on one side)
check_both_in!(Opq2, expr!(Mul N0 ON), N0);
check_both_in!(Opq2, expr!(Mul N1 ON), ON);
