#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![allow(incomplete_features)]

use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul, Sub},
};

use num::{BigInt, CanContain, Num};
use type_bool::{If, True};

mod num;
mod type_bool;

#[derive(Clone, Copy, Debug, Hash)]
// #[repr(transparent)]
pub struct Int<const MIN: BigInt, const MAX: BigInt, T>(T);

impl<const MIN: BigInt, const MAX: BigInt, T: CanContain<MIN, MAX>> Int<MIN, MAX, T> {
    pub fn new(v: T) -> Option<Self> {
        (v.into() >= MIN && v.into() <= MAX).then_some(Self(v))
    }

    pub fn into_range<
        const MIN2: BigInt,
        const MAX2: BigInt,
        T2: CanContain<MIN2, MAX2> + TryFrom<T>,
    >(
        self,
    ) -> Int<MIN2, MAX2, T2>
    where
        If<{ MIN2 <= MIN }>: True,
        If<{ MAX2 >= MAX }>: True,
    {
        Int(T2::try_from(self.0).ok().unwrap())
    }

    pub fn into<T2: CanContain<MIN, MAX> + TryFrom<T>>(self) -> Int<MIN, MAX, T2> {
        Int(T2::try_from(self.0).ok().unwrap())
    }

    pub fn into_prim<U: From<T>>(self) -> U {
        U::from(self.0)
    }

    pub fn clamp<const MIN2: BigInt, const MAX2: BigInt>(self) -> Int<MIN2, MAX2, T> {
        let v = self.0.into().min(MAX2).max(MIN2);
        Int(T::try_from(v).ok().unwrap())
    }

    pub fn try_into_range<const MIN2: BigInt, const MAX2: BigInt>(
        self,
    ) -> Option<Int<MIN2, MAX2, T>> {
        let v = self.0.into();
        (v >= MIN2 && v <= MAX2).then(|| Int(T::try_from(v).ok().unwrap()))
    }
}

pub type ToInt<T: Num> = Int<{ T::MIN_BIG_INT }, { T::MAX_BIG_INT }, T>;

impl<
        const MIN1: BigInt,
        const MAX1: BigInt,
        const MIN2: BigInt,
        const MAX2: BigInt,
        T: Add<T> + CanContain<{ MIN1 + MIN2 }, { MAX1 + MAX2 }>,
    > Add<Int<MIN2, MAX2, T>> for Int<MIN1, MAX1, T>
{
    type Output = Int<{ MIN1 + MIN2 }, { MAX1 + MAX2 }, <T as Add<T>>::Output>;

    fn add(self, rhs: Int<MIN2, MAX2, T>) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl<
        const MIN1: BigInt,
        const MAX1: BigInt,
        const MIN2: BigInt,
        const MAX2: BigInt,
        T: Sub<T> + CanContain<{ MIN1 - MAX2 }, { MAX1 - MIN2 }>,
    > Sub<Int<MIN2, MAX2, T>> for Int<MIN1, MAX1, T>
{
    type Output = Int<{ MIN1 - MAX2 }, { MAX1 - MIN2 }, <T as Sub<T>>::Output>;

    fn sub(self, rhs: Int<MIN2, MAX2, T>) -> Self::Output {
        Int(self.0 - rhs.0)
    }
}

const fn min(a: BigInt, b: BigInt) -> BigInt {
    if a <= b {
        a
    } else {
        b
    }
}

const fn max(a: BigInt, b: BigInt) -> BigInt {
    if a >= b {
        a
    } else {
        b
    }
}

pub const fn mul_min(min1: BigInt, max1: BigInt, min2: BigInt, max2: BigInt) -> BigInt {
    let a = min1 * min2;
    let b = min1 * max2;
    let c = max1 * min2;
    let d = max1 * max2;
    min(a, min(b, min(c, d)))
}

pub const fn mul_max(min1: BigInt, max1: BigInt, min2: BigInt, max2: BigInt) -> BigInt {
    let a = min1 * min2;
    let b = min1 * max2;
    let c = max1 * min2;
    let d = max1 * max2;
    max(a, max(b, max(c, d)))
}

impl<
        const MIN1: BigInt,
        const MAX1: BigInt,
        const MIN2: BigInt,
        const MAX2: BigInt,
        T: Mul<T>
            + CanContain<{ mul_min(MIN1, MAX1, MIN2, MAX2) }, { mul_max(MIN1, MAX1, MIN2, MAX2) }>,
    > Mul<Int<MIN2, MAX2, T>> for Int<MIN1, MAX1, T>
{
    type Output = Int<
        { mul_min(MIN1, MAX1, MIN2, MAX2) },
        { mul_max(MIN1, MAX1, MIN2, MAX2) },
        <T as Mul<T>>::Output,
    >;

    fn mul(self, rhs: Int<MIN2, MAX2, T>) -> Self::Output {
        Int(self.0 * rhs.0)
    }
}

impl<
        const MIN1: BigInt,
        const MAX1: BigInt,
        const MIN2: BigInt,
        const MAX2: BigInt,
        T1: PartialEq<T2>,
        T2,
    > PartialEq<Int<MIN2, MAX2, T2>> for Int<MIN1, MAX1, T1>
{
    fn eq(&self, other: &Int<MIN2, MAX2, T2>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<const MIN: BigInt, const MAX: BigInt, T: Eq> Eq for Int<MIN, MAX, T> {}

impl<
        const MIN1: BigInt,
        const MAX1: BigInt,
        const MIN2: BigInt,
        const MAX2: BigInt,
        T1: PartialOrd<T2>,
        T2,
    > PartialOrd<Int<MIN2, MAX2, T2>> for Int<MIN1, MAX1, T1>
{
    fn partial_cmp(&self, other: &Int<MIN2, MAX2, T2>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<const MIN: BigInt, const MAX: BigInt, T: Ord> Ord for Int<MIN, MAX, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<const MIN: BigInt, const MAX: BigInt, T: Display> Display for Int<MIN, MAX, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn int<const V: BigInt, T: CanContain<V, V>>() -> Int<V, V, T> {
    Int::new(T::try_from(V).ok().unwrap()).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{int, Int, ToInt};

    #[test]
    fn test_mul_add() {
        let a = int::<10, u8>();
        let b = int::<5, i32>();
        let x: i32 = int::<200, u8>().into_prim();
        let c: Int<15, 15, u8> = a + b.into();
        let c2: Int<5, 5, u8> = a - b.into();
        let d: Int<10, 20, i32> = c.into_range();
        let e: Int<50, 50, u8> = a * b.into();
        let f: Int<_, _, i32> = e.into();
        let g: Int<10, 270, i16> = e.into_range();
        let h: Int<10, 271, i16> = g.into_range();
        let i: Int<_, _, _> = h * c.into() * b.into();
        println!("{c2}, {d}, {f}, {g}, {h}, {i}");
    }

    #[test]
    fn test_ord() {
        let a = int::<10, u8>();
        let b = int::<5, i32>();
        assert!(a > b.into());
    }
}
