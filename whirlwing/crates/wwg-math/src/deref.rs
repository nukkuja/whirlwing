// Huge thanks to nalgebra crate for inspiration for this code!
// https://github.com/dimforge/nalgebra

macro_rules! coords_struct {
    ($name: ident, $($params: ident),+) => {
        #[allow(clippy::upper_case_acronyms)]
        #[repr(C)]
        pub struct $name {
            $(pub $params: f32),+
        }
    };
    ($name: ident, $($params: ident),+,) => {
        #[allow(clippy::upper_case_acronyms)]
        #[repr(C)]
        pub struct $name {
            $(pub $params: f32),+
        }
    }
}

macro_rules! impl_deref {
    ($type: ty, $target: ty) => {
        impl std::ops::Deref for $type {
            type Target = $target;
            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.ptr() as *const Self::Target) }
            }
        }
        impl std::ops::DerefMut for $type {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self.mut_ptr() as *mut Self::Target) }
            }
        }
    };
}

coords_struct!(XYZ, x, y, z);
coords_struct!(XYZW, x, y, z, w);

// Change this if you want row-major matrix order
coords_struct!(M4, x1, y1, z1, w1, x2, y2, z2, w2, x3, y3, z3, w3, x4, y4, z4, w4,);

use crate::core::*;

impl_deref!(Vector3, XYZ);
impl_deref!(Vector4, XYZW);
impl_deref!(Matrix4, M4);
