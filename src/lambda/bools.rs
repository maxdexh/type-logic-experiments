use super::*;

// True and False are usually represented as the functions that
// ignore their second and first argument, respectively.
//
// Alternatively, consider Tern C T F := if C then T else F
// We pick True = Tern True, False = Tern False
pub type True = expr! { X: Y: X };
pub type False = expr! { X: Y: Y };

// This definition means the ternary function becomes trivial.
// In particular Tern = Identity (modulo argument laziness)
check_both!(expr!(/* Tern */ True OX OY), OX);
check_both!(expr!(/* Tern */ False OX OY), OY);

// Since we have the ternary function, we can trivially define all
// our favorite logical operators
type LNot = expr! { B: B False True }; // LNot B = if B then False else True 
type LAnd = expr! { B: C: B C B };
type LOr = expr! { B: C: B B C };

// For `Simp`, these equalities are symbolic without opaques
check_both_in!(Opq2, expr!(LNot True), False);
check_both_in!(Opq2, expr!(LNot False), True);
check_both_in!(Opq2, expr!(LAnd True False), False);
check_both_in!(Opq2, expr!(LOr True False), True);
