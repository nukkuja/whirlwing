// Huge thanks to nalgebra crate for inspiration for this code!
// https://github.com/dimforge/nalgebra

macro_rules! coords_struct {
    ($name: ident, $($params: ident),+) => {
        #[repr(C)]
        pub struct $name {
            $(pub $params: f32),+
        }
    };
    ($name: ident, $($params: ident),+,) => {
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
                unsafe { &*(self.data.as_ptr() as *const Self::Target) }
            }
        }
        impl std::ops::DerefMut for $type {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self.data.as_mut_ptr() as *mut Self::Target) }
            }
        }
    }
}

coords_struct!(XYZ, x, y, z);
coords_struct!(XYZW, x, y, z, w);
coords_struct!(
    M4,
    x1, x2, x3, x4,
    y1, y2, y3, y4,
    z1, z2, z3, z4,
    w1, w2, w3, w4,
);

use crate::core::*;

impl_deref!(Vector3, XYZ);
impl_deref!(Vector4, XYZW);
impl_deref!(Matrix4, M4);