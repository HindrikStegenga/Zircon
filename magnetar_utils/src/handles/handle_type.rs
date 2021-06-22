use num_traits::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;

pub trait HandleType:
    Debug
    + Copy
    + Clone
    + Send
    + ToPrimitive
    + FromPrimitive
    + Bounded
    + PrimInt
    + PartialEq
    + Num
    + Unsigned
    + AddAssign
    + Eq
    + PartialEq
    + Hash
{
}

impl HandleType for u8 {}
impl HandleType for u16 {}
impl HandleType for u32 {}
impl HandleType for u64 {}
impl HandleType for u128 {}
impl HandleType for usize {}
