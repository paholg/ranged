#![no_std]

#[macro_use]
pub extern crate typenum;

use core::marker::PhantomData;
use typenum::{Integer, Unsigned};

/**
A type constrained by the `Min` and `Max` parameters.

# Example

```rust
use ranged::typenum::{Z0, P30};
use ranged::Ranged;

let some_val: Ranged<i64, Z0, P30> = Ranged::new(10);
let some_other: Ranged<i64, Z0, P30> = Ranged::new(12);
let new_val = some_val + some_other;
// This will print: "10 + 12 = 22"
println!("{} + {} = {}", some_val, some_other, new_val);
// This will panic in debug mode, but work in release mode:
// let new_negative = some_val - some_other;
```
*/
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ranged<T, Min, Max> {
    val: T,
    _marker: PhantomData<(Min, Max)>,
}

pub trait RangedBuilder<Min, Max>
    where Self: Sized
{
    fn new(val: Self) -> Ranged<Self, Min, Max>;
}

use core::fmt;
impl<T, Min, Max> fmt::Display for Ranged<T, Min, Max>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.val.fmt(f)
    }
}

macro_rules! impl_ranged_builder {
    ($trait:ident; $($t:ident, $fun:ident),*) => (
        $(
            #[cfg(debug_assertions)]
            impl<Min, Max> RangedBuilder<Min, Max> for $t where Min: $trait, Max: $trait {
                #[inline]
                fn new(val: $t) -> Ranged<$t, Min, Max> {
                    if val > Max::$fun() || val < Min::$fun() {
                        panic!("value is out of range");
                    }
                    Ranged { val: val, _marker: PhantomData }
                }
            }
        )*
    );
}

impl_ranged_builder!(Integer; i8, to_i8, i16, to_i16, i32, to_i32, i64, to_i64);
impl_ranged_builder!(Unsigned; u8, to_u8, u16, to_u16, u32, to_u32, u64, to_u64);

#[cfg(not(debug_assertions))]
impl<T, Min, Max> RangedBuilder<Min, Max> for T {
    #[inline]
    fn new(val: T) -> Ranged<T, Min, Max> {
        Ranged {
            val: val,
            _marker: PhantomData,
        }
    }
}

impl<T, Min, Max> Ranged<T, Min, Max>
    where T: RangedBuilder<Min, Max>
{
    #[inline]
    pub fn new(val: T) -> Ranged<T, Min, Max> {
        RangedBuilder::new(val)
    }
}



use core::ops::*;

macro_rules! impl_binary_op {
    ($($trait:ident $fun:ident $op:tt),*) => (
        $(
            impl<T, U, Min, Max> $trait<Ranged<U, Min, Max>> for Ranged<T, Min, Max>
                where T: $trait<U>, op!(T $op U): RangedBuilder<Min, Max>
            {
                type Output = Ranged<op!(T $op U), Min, Max>;
                #[inline]
                fn $fun(self, rhs: Ranged<U, Min, Max>) -> Self::Output {
                    Ranged::new(self.val $op rhs.val)
                }
            }
        )*
    );
}

impl_binary_op!(Add add +, Sub sub -, Mul mul *, Div div /, Rem rem %,
                BitAnd bitand &, BitOr bitor |, BitXor bitxor ^);
