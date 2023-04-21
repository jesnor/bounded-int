use crate::type_bool::{If, True};

#[repr(u8)]
pub enum NumType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    I128,
}

pub type BigInt = i128;

pub trait Num: Copy + Eq + Ord + TryFrom<BigInt> + Into<BigInt> {
    const MIN: Self;
    const MAX: Self;
    const MIN_BIG_INT: BigInt;
    const MAX_BIG_INT: BigInt;
    const NUM_TYPE: NumType;
}

impl Num for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
    const MIN_BIG_INT: BigInt = u8::MIN as BigInt;
    const MAX_BIG_INT: BigInt = u8::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::U8;
}

impl Num for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
    const MIN_BIG_INT: BigInt = u16::MIN as BigInt;
    const MAX_BIG_INT: BigInt = u16::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::U16;
}

impl Num for u32 {
    const MIN: Self = u32::MIN;
    const MAX: Self = u32::MAX;
    const MIN_BIG_INT: BigInt = u32::MIN as BigInt;
    const MAX_BIG_INT: BigInt = u32::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::U32;
}

impl Num for u64 {
    const MIN: Self = u64::MIN;
    const MAX: Self = u64::MAX;
    const MIN_BIG_INT: BigInt = u64::MIN as BigInt;
    const MAX_BIG_INT: BigInt = u64::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::U64;
}

impl Num for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
    const MIN_BIG_INT: BigInt = i8::MIN as BigInt;
    const MAX_BIG_INT: BigInt = i8::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::I8;
}

impl Num for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
    const MIN_BIG_INT: BigInt = i16::MIN as BigInt;
    const MAX_BIG_INT: BigInt = i16::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::I16;
}

impl Num for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;
    const MIN_BIG_INT: BigInt = i32::MIN as BigInt;
    const MAX_BIG_INT: BigInt = i32::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::I32;
}

impl Num for i64 {
    const MIN: Self = i64::MIN;
    const MAX: Self = i64::MAX;
    const MIN_BIG_INT: BigInt = i64::MIN as BigInt;
    const MAX_BIG_INT: BigInt = i64::MAX as BigInt;
    const NUM_TYPE: NumType = NumType::I64;
}

impl Num for i128 {
    const MIN: Self = i128::MIN;
    const MAX: Self = i128::MAX;
    const MIN_BIG_INT: BigInt = BigInt::MIN;
    const MAX_BIG_INT: BigInt = BigInt::MAX;
    const NUM_TYPE: NumType = NumType::I128;
}

pub const fn can_contain<T: Num>(min: BigInt, max: BigInt) -> bool {
    min <= max && min >= T::MIN_BIG_INT && max <= T::MAX_BIG_INT
}

pub trait CanContain<const MIN: BigInt, const MAX: BigInt>: Num {}

impl<const MIN: BigInt, const MAX: BigInt, T: Num + Into<BigInt>> CanContain<MIN, MAX> for T where
    If<{ can_contain::<T>(MIN, MAX) }>: True
{
}

// pub const fn smallest_num_type(min: BigInt, max: BigInt) -> NumType {
//     if can_contain::<u8>(min, max) {
//         NumType::U8
//     } else if can_contain::<u16>(min, max) {
//         NumType::U16
//     } else if can_contain::<u32>(min, max) {
//         NumType::U32
//     } else if can_contain::<u64>(min, max) {
//         NumType::U64
//     } else if can_contain::<i8>(min, max) {
//         NumType::I8
//     } else if can_contain::<i16>(min, max) {
//         NumType::I16
//     } else if can_contain::<i32>(min, max) {
//         NumType::I32
//     } else if can_contain::<i64>(min, max) {
//         NumType::I64
//     } else {
//         NumType::I128
//     }
// }

// pub type SmallestNumType<const MIN: BigInt, const MAX: BigInt> =
//     <SNumTypeToType<{ smallest_num_type(MIN, MAX) as u8 }> as NumTypeToType>::T;

pub trait NumTypeToType {
    type T: Num;
}

pub struct SNumTypeToType<const N: u8>;

impl NumTypeToType for SNumTypeToType<{ NumType::U8 as u8 }> {
    type T = u8;
}

impl NumTypeToType for SNumTypeToType<{ NumType::U16 as u8 }> {
    type T = u16;
}

impl NumTypeToType for SNumTypeToType<{ NumType::U32 as u8 }> {
    type T = u32;
}

impl NumTypeToType for SNumTypeToType<{ NumType::U64 as u8 }> {
    type T = u64;
}

impl NumTypeToType for SNumTypeToType<{ NumType::I8 as u8 }> {
    type T = i8;
}

impl NumTypeToType for SNumTypeToType<{ NumType::I16 as u8 }> {
    type T = i16;
}

impl NumTypeToType for SNumTypeToType<{ NumType::I32 as u8 }> {
    type T = i32;
}

impl NumTypeToType for SNumTypeToType<{ NumType::I64 as u8 }> {
    type T = i64;
}

impl NumTypeToType for SNumTypeToType<{ NumType::I128 as u8 }> {
    type T = i128;
}
