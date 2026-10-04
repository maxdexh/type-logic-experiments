use super::*;

vars!(A = 0, B = 1);

pub type Rev = expr! {
    fn A =>
    (B = EmptyList);
    while A {
        (B = Push B (Last A));
        (A = Pop A)
    } else {
        B
    }
};

#[cfg(false)] // FIXME: Broken
mod tests {
    use super::*;
    use crate::utils::check_type_eq;

    check_type_eq!(
        Eval<expr! { Rev (Push EmptyList 1) }>,
        Eval<expr! { Push EmptyList 1 }>,
    );
    check_type_eq!(
        Eval<expr! { Rev (Push (Push EmptyList 1) 2) }>,
        Eval<expr! { Push (Push EmptyList 2) 1 }>,
    );
}
