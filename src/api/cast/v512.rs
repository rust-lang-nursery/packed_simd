//! `FromCast` and `IntoCast` implementations for portable 512-bit wide vectors
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::*;

impl_from_cast!(i8x64: u8x64, m8x64);
impl_from_cast!(u8x64: i8x64, m8x64);
impl_from_cast_mask!(m8x64: i8x64, u8x64);

impl_from_cast!(i16x32: i8x32, u8x32, m8x32, u16x32, m16x32);
impl_from_cast!(u16x32: i8x32, u8x32, m8x32, i16x32, m16x32);
impl_from_cast_mask!(m16x32: i8x32, u8x32, m8x32, i16x32, u16x32);

impl_from_cast!(
    i32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, u32x16, f32x16, m32x16
);
impl_from_cast!(
    u32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, f32x16, m32x16
);
impl_from_cast!(
    f32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, m32x16
);
impl_from_cast_mask!(
    m32x16: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16
);

impl_from_cast!(
    i64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        u64x8, f64x8, m64x8
);
impl_from_cast!(
    u64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        i64x8, f64x8, m64x8
);
impl_from_cast!(
    f64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        i64x8, u64x8, m64x8
);
impl_from_cast_mask!(
    m64x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
        i64x8, u64x8, f64x8
);

impl_from_cast!(
    i128x4: i8x4, u8x4, m8x4, i16x4, u16x4, m16x4, i32x4, u32x4, f32x4, m32x4,
    i64x4, u64x4, f64x4, m64x4, u128x4, m128x4
);
impl_from_cast!(
    u128x4: i8x4, u8x4, m8x4, i16x4, u16x4, m16x4, i32x4, u32x4, f32x4, m32x4,
    i64x4, u64x4, f64x4, m64x4, i128x4, m128x4
);
impl_from_cast_mask!(
    m128x4: i8x4, u8x4, m8x4, i16x4, u16x4, m16x4, i32x4, u32x4, f32x4, m32x4,
    i64x4, u64x4, m64x4, f64x4, i128x4, u128x4
);
