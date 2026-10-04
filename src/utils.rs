macro_rules! __mk_nat_mac {
    ($nty:ty, $($i:literal)*) => {
        pub trait __BitToBit<const B: u16> {
            type Out;
        }
        impl __BitToBit<0> for () {
            type Out = _0;
        }
        impl __BitToBit<1> for () {
            type Out = _1;
        }
        pub type __P<const _I: $nty, N, const __B: $nty> =
            <N as Nat>::PushBit<<() as __BitToBit<__B>>::Out>;

        #[allow(unused)]
        macro_rules! nat {
            ($n:expr) => {
                $(__P<$i,)*
                _0
                $(, { ($n >> $i) & 1 }>)*
            };
        }
        #[allow(unused)]
        pub(crate) use nat;
    };
}
pub(crate) use __mk_nat_mac;

macro_rules! mk_nat {
    ({
        trait Nat {}

        trait Bit { $(
            type $bit_asc:ident<$($bit_gp:ident: $bit_gb:path),*>: $bit_asb:path = If<
                Self,
                $bit_asc_1:ty,
                $bit_asc_0:ty $(,)?
            >;
        )* }
    }) => {
        pub trait Nat {
            type Par: Bit;
            type Pop: Nat;
            type IsZero: Bit;
            type IsNonZero: Bit;
            type IsOne: Bit;
            type Eq<R: Nat>: Bit;
            type PushBit<B: Bit>: Nat;
        }
        pub trait Bit: Nat {
            type Not: Bit;
            type Xnor<R: Bit>: Bit;
            type AndEq<L: Nat, R: Nat>: Bit;
            $(type $bit_asc<$($bit_gp: $bit_gb),*>: $bit_asb;)*
        }

        pub struct _0;
        pub struct _1;

        impl Bit for _0 {
            type Not = _1;
            type Xnor<R: Bit> = R::Not;
            type AndEq<L: Nat, R: Nat> = _0;
            $(type $bit_asc<$($bit_gp: $bit_gb),*> = $bit_asc_0;)*
        }
        impl Bit for _1 {
            type Not = _0;
            type Xnor<R: Bit> = R;
            type AndEq<L: Nat, R: Nat> = L::Eq<R>;
            $(type $bit_asc<$($bit_gp: $bit_gb),*> = $bit_asc_1;)*
        }

        const _: () = {
            pub trait __PNatHelper: Nat {}

            impl Nat for _0 {
                type Par = Self;
                type Pop = Self;
                type IsZero = _1;
                type IsNonZero = _0;
                type IsOne = _0;
                type PushBit<B: Bit> = B;

                type Eq<R: Nat> = R::IsZero;
            }
            impl Nat for _1 {
                type Par = Self;
                type Pop = _0;
                type IsZero = _0;
                type IsNonZero = _1;
                type PushBit<B: Bit> = (Self, B);

                type IsOne = _1;
                type Eq<R: Nat> = R::IsOne;
            }
            impl<Pop: __PNatHelper, Par: Bit> Nat for (Pop, Par) {
                type Par = Par;
                type Pop = Pop;
                type IsZero = _0;
                type IsNonZero = _1;
                type PushBit<B: Bit> = (Self, B);

                type IsOne = _0;
                type Eq<R: Nat> = <Par::Xnor<R::Par> as Bit>::AndEq<R::Pop, Pop>;
            }

            impl __PNatHelper for _1 {}
            impl<Pop: __PNatHelper, Par: Bit> __PNatHelper for (Pop, Par) {}
        };

        $crate::utils::__mk_nat_mac! {
            u16,
            15 14 13 12
            11 10 9 8
            7 6 5 4
            3 2 1 0
        }
    };
}
pub(crate) use mk_nat;

#[cfg_attr(not(test), allow(unused))]
macro_rules! check_type_eq {
    ($lhs:ty, $rhs:ty $(,)?) => {
        const _: () = {
            #[allow(unused)]
            fn _func(x: $lhs) -> $rhs {
                x
            }
        };
    };
}
#[cfg_attr(not(test), allow(unused))]
pub(crate) use check_type_eq;
