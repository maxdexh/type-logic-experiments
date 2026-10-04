use super::*;

type ZInner = expr!(X: F (Y: X X Y));
pub type ZComb = expr!(F: ZInner ZInner);
