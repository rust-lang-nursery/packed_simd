//! Implements vertical (lane-wise) floating-point `fma`.

macro_rules! impl_math_float_fma {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Fused multiply add: `self * y + z`
            ///
            /// Most architectures which have support for FMA
            /// also have an equivalent version of this function,
            /// fused multiply subtract (`self * y - z`).
            /// Simply negating the second parameter of this function
            /// will make the compiler generate it.
            #[inline]
            pub fn fma(self, y: Self, z: Self) -> Self {
                use crate::codegen::math::float::fma::Fma;
                Fma::fma(self, y, z)
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                pub mod [$id _math_fma] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn fma() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        let t = $id::splat(2 as $elem_ty);
                        let t3 = $id::splat(3 as $elem_ty);
                        let f = $id::splat(4 as $elem_ty);

                        assert_eq!(z, z.fma(z, z));
                        assert_eq!(o, o.fma(o, z));
                        assert_eq!(o, o.fma(z, o));
                        assert_eq!(o, z.fma(o, o));

                        assert_eq!(t, o.fma(o, o));
                        assert_eq!(t, o.fma(t, z));
                        assert_eq!(t, t.fma(o, z));

                        assert_eq!(f, t.fma(t, z));
                        assert_eq!(f, t.fma(o, t));
                        assert_eq!(t3, t.fma(o, o));
                    }
                }
            }
        }
    };
}
