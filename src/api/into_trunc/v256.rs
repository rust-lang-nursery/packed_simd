//! `FromTrunc` and `IntoTrunc` implementations for portable 256-bit wide vectors
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::*;

impl_from_trunc!(i8x32: u8x32, m8x32, i16x32, u16x32, m16x32);
impl_from_trunc!(u8x32: i8x32, m8x32, i16x32, u16x32, m16x32);
impl_from_trunc_mask!(m8x32: i8x32, u8x32, i16x32, u16x32, m16x32);

impl_from_trunc!(
    i16x16: i8x16, u8x16, m8x16, u16x16, m16x16,
    i32x16, u32x16, f32x16, m32x16
);
impl_from_trunc!(
    u16x16: i8x16, u8x16, m8x16, i16x16, m16x16,
    i32x16, u32x16, f32x16, m32x16
);
impl_from_trunc_mask!(
    m16x16: i8x16, u8x16, m8x16, i16x16, u16x16,
    i32x16, u32x16, f32x16, m32x16
);

impl_from_trunc!(
    i32x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, u32x8, f32x8, m32x8,
        i64x8, u64x8, f64x8, m64x8
);
impl_from_trunc!(
    u32x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, f32x8, m32x8,
        i64x8, u64x8, f64x8, m64x8
);
impl_from_trunc!(
    f32x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, m32x8,
        i64x8, u64x8, f64x8, m64x8
);
impl_from_trunc_mask!(
    m32x8: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8,
        i64x8, u64x8, f64x8, m64x8
);

impl_from_trunc!(
    i64x4: i8x4, u8x4, m8x4, i16x4, u16x4, m16x4, i32x4, u32x4, f32x4, m32x4,
        u64x4, f64x4, m64x4, i128x4, u128x4, m128x4
);
impl_from_trunc!(
    u64x4: i8x4, u8x4, m8x4, i16x4, u16x4, m16x4, i32x4, u32x4, f32x4, m32x4,
        i64x4, f64x4, m64x4, i128x4, u128x4, m128x4
);
impl_from_trunc!(
    f64x4: i8x4, u8x4, m8x4, i16x4, u16x4, m16x4, i32x4, u32x4, f32x4, m32x4,
        i64x4, u64x4, m64x4, i128x4, u128x4, m128x4
);
impl_from_trunc_mask!(
    m64x4: i8x4, u8x4, m8x4, i16x4, u16x4, m16x4, i32x4, u32x4, f32x4, m32x4,
        i64x4, u64x4, f64x4, i128x4, u128x4, m128x4
);
