mod f32x4 {
    #![allow(unused)]
    use packed_simd::*;
    use stdsimd_test::assert_instr;

    #[inline]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature(enable = "sse,fma")
    )]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"), assert_instr(vfmadd)
    )]
    unsafe fn fused_multiply_add(a: f32x4, b: f32x4, c: f32x4) -> f32x4 {
        a.mul_add(b, c)
    }

    #[inline]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature(enable = "sse,fma")
    )]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"), assert_instr(vfmsub)
    )]
    unsafe fn fused_multiply_sub(a: f32x4, b: f32x4, c: f32x4) -> f32x4 {
        a.mul_add(b, -c)
    }

    #[inline]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature(enable = "sse,fma")
    )]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"), assert_instr(vfnmadd)
    )]
    unsafe fn fused_negate_multiply_add(
        a: f32x4, b: f32x4, c: f32x4,
    ) -> f32x4 {
        a.mul_add(-b, c)
    }

    #[inline]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature(enable = "sse,fma")
    )]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"), assert_instr(vfnmsub)
    )]
    unsafe fn fused_negate_multiply_sub(
        a: f32x4, b: f32x4, c: f32x4,
    ) -> f32x4 {
        a.mul_add(-b, -c)
    }

    #[inline]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature(enable = "sse,fma")
    )]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        assert_instr(vfmaddsub)
    )]
    unsafe fn fused_multiply_add_sub(a: f32x4, b: f32x4, c: f32x4) -> f32x4 {
        let add = a.mul_add(b, c);
        let sub = a.mul_add(b, -c);

        m32x4::new(false, true, false, true).select(add, sub)
    }

    #[inline]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature(enable = "sse,fma")
    )]
    #[cfg_attr(
        any(target_arch = "x86", target_arch = "x86_64"),
        assert_instr(vfmsubadd)
    )]
    unsafe fn fused_multiply_sub_add(a: f32x4, b: f32x4, c: f32x4) -> f32x4 {
        let add = a.mul_add(b, c);
        let sub = a.mul_add(b, -c);

        m32x4::new(true, false, true, false).select(add, sub)
    }
}
