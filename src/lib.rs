#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![allow(incomplete_features)]

use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul},
};

use num::{BigInt, CanContain};
use type_bool::{If, True};

mod num;
mod type_bool;

#[derive(Clone, Copy, Debug, Hash)]
#[repr(transparent)]
pub struct Int<const MIN: BigInt, const MAX: BigInt, T>(T);

impl<const MIN: BigInt, const MAX: BigInt, T: CanContain<MIN, MAX>> Int<MIN, MAX, T> {
    pub fn new(v: T) -> Self {
        assert!(v.into() >= MIN);
        assert!(v.into() <= MAX);
        Self(v)
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
}

impl<
        const MIN1: BigInt,
        const MAX1: BigInt,
        const MIN2: BigInt,
        const MAX2: BigInt,
        T: Add<T> + CanContain<{ MIN1 + MIN2 }, { MAX1 + MAX2 }>,
    > Add<Int<MIN2, MAX2, T>> for Int<MIN1, MAX1, T>
where
    [(); (MIN1 + MIN2) as usize]:,
    [(); (MAX1 + MAX2) as usize]:,
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
        T: Mul<T> + CanContain<{ MIN1 * MIN2 }, { MAX1 * MAX2 }>,
    > Mul<Int<MIN2, MAX2, T>> for Int<MIN1, MAX1, T>
where
    [(); (MIN1 * MIN2) as usize]:,
    [(); (MAX1 * MAX2) as usize]:,
{
    type Output = Int<{ MIN1 * MIN2 }, { MAX1 * MAX2 }, <T as Mul<T>>::Output>;

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
    Int::new(T::try_from(V).ok().unwrap())
}

#[cfg(test)]
mod test {
    use crate::{int, Int};

    #[test]
    fn test_mul_add() {
        let a = int::<10, u8>();
        let b = int::<5, i32>();
        let c: Int<15, 15, u8> = a + b.into();
        let d: Int<10, 20, i32> = c.into_range();
        let e: Int<50, 50, u8> = a * b.into();
        let f: Int<_, _, i32> = e.into();
        let g: Int<10, 270, i16> = e.into_range();
        let h: Int<10, 271, i16> = g.into_range();
        let i: Int<_, _, _> = h * c.into();
        println!("{d}, {f}, {g}, {h}, {i}");
    }

    fn test_ord() {
        let a = int::<10, u8>();
        let b = int::<5, i32>();
        assert!(a > b.into());
    }
}
